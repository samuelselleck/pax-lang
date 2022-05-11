use core::option::Option;
use core::option::Option::Some;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use kurbo::BezPath;
use piet::RenderContext;

use pax_core::{RenderNode, RenderNodePtrList, RenderTreeContext, RenderNodePtr, InstantiationArgs, HandlerRegistry};
use pax_properties_coproduct::TypesCoproduct;
use pax_runtime_api::{Transform2D, Size, PropertyInstance, Size2D};
use pax_message::{AnyCreatePatch, FramePatch};

/// A primitive that gathers children underneath a single render node with a shared base transform,
/// like [`Group`], except [`Frame`] has the option of clipping rendering outside
/// of its bounds.
///
/// If clipping or the option of clipping is not required,
/// a [`Group`] will generally be a more performant and otherwise-equivalent
/// to [`Frame`], since `[Frame]` creates a clipping mask.
pub struct FrameInstance<R: RenderContext> {
    pub instance_id: u64,
    pub children: RenderNodePtrList<R>,
    pub size: Size2D,
    pub transform: Rc<RefCell<dyn PropertyInstance<Transform2D>>>,

    last_patches: HashMap<Vec<u64>, FramePatch>,
}

impl<R: 'static + RenderContext> RenderNode<R> for FrameInstance<R> {
    fn get_instance_id(&self) -> u64 {
        self.instance_id
    }
    
    fn instantiate(args: InstantiationArgs<R>) -> Rc<RefCell<Self>> where Self: Sized {

        let mut instance_registry = args.instance_registry.borrow_mut();
        let instance_id = instance_registry.mint_id();
        let ret = Rc::new(RefCell::new(
            Self {
                instance_id,
                children: args.children.expect("Frame expects primitive_children, even if empty Vec"),
                size: Rc::new(RefCell::new(args.size.expect("Frame requires size"))),
                transform: args.transform,
                last_patches: HashMap::new(),
            }
        ));

        instance_registry.register(instance_id, Rc::clone(&ret) as RenderNodePtr<R>);
        ret
    }

    fn compute_native_patches(&mut self, rtc: &mut RenderTreeContext<R>, size_calc: (f64, f64), transform_coeffs: Vec<f64>) {

        let mut new_message : FramePatch = Default::default();
        new_message.id_chain = rtc.get_id_chain(self.instance_id);
        if ! self.last_patches.contains_key(&new_message.id_chain) {
            let mut patch = FramePatch::default();
            patch.id_chain = new_message.id_chain.clone();
            self.last_patches.insert(new_message.id_chain.clone(),patch);
        }
        let last_patch = self.last_patches.get_mut( &new_message.id_chain).unwrap();
        let mut has_any_updates = false;

        let val = size_calc.0;
        let is_new_value = match &last_patch.size_x {
            Some(cached_value) => {
                !val.eq(cached_value)
            },
            None => {
                true
            },
        };
        if is_new_value {
            new_message.size_x = Some(val.clone());
            last_patch.size_x = Some(val.clone());
            has_any_updates = true;
        }

        let val = size_calc.1;
        let is_new_value = match &last_patch.size_y {
            Some(cached_value) => {
                !val.eq(cached_value)
            },
            None => {
                true
            },
        };
        if is_new_value {
            new_message.size_y = Some(val.clone());
            last_patch.size_y = Some(val.clone());
            has_any_updates = true;
        }

        let latest_transform = transform_coeffs;
        let is_new_transform = match &last_patch.transform {
            Some(cached_transform) => {
                latest_transform.iter().enumerate().any(|(i,elem)|{
                    *elem != cached_transform[i]
                })
            },
            None => {
                true
            },
        };
        if is_new_transform {
            new_message.transform = Some(latest_transform.clone());
            last_patch.transform = Some(latest_transform.clone());
            has_any_updates = true;
        }

        if has_any_updates {
            (*rtc.engine.runtime).borrow_mut().enqueue_native_message(
                pax_message::NativeMessage::FrameUpdate(new_message)
            );
        }
    }

    fn get_rendering_children(&self) -> RenderNodePtrList<R> {
        Rc::clone(&self.children)
    }

    fn get_size(&self) -> Option<Size2D> {
        Some(Rc::clone(&self.size))
    }

    fn get_transform(&mut self) -> Rc<RefCell<dyn PropertyInstance<Transform2D>>> { Rc::clone(&self.transform) }

    fn compute_properties(&mut self, rtc: &mut RenderTreeContext<R>) {
        let mut size = &mut *self.size.as_ref().borrow_mut();

        if let Some(new_size) = rtc.compute_vtable_value(size[0]._get_vtable_id()) {
            let new_value = if let TypesCoproduct::Size(v) = new_size { v } else { unreachable!() };
            size[0].set(new_value);
        }

        if let Some(new_size) = rtc.compute_vtable_value(size[1]._get_vtable_id()) {
            let new_value = if let TypesCoproduct::Size(v) = new_size { v } else { unreachable!() };
            size[1].set(new_value);
        }

        let mut transform = &mut *self.transform.as_ref().borrow_mut();
        if let Some(new_transform) = rtc.compute_vtable_value(transform._get_vtable_id()) {
            let new_value = if let TypesCoproduct::Transform2D(v) = new_transform { v } else { unreachable!() };
            transform.set(new_value);
        }
    }

    fn handle_pre_render(&mut self, rtc: &mut RenderTreeContext<R>, rc: &mut R) {
        // construct a BezPath of this frame's bounds * its transform,
        // then pass that BezPath into rc.clip() [which pushes a clipping context to a piet-internal stack]
        //TODO:  if clipping is TURNED OFF for this Frame, don't do any of this

        let transform = rtc.transform;
        let bounding_dimens = rtc.bounds;

        let width: f64 =  bounding_dimens.0;
        let height: f64 =  bounding_dimens.1;

        let mut bez_path = BezPath::new();
        bez_path.move_to((0.0, 0.0));
        bez_path.line_to((width , 0.0));
        bez_path.line_to((width , height ));
        bez_path.line_to((0.0, height));
        bez_path.line_to((0.0,0.0));
        bez_path.close_path();

        let transformed_bez_path = transform * bez_path;
        rc.save().unwrap(); //our "save point" before clipping — restored to in the post_render
        rc.clip(transformed_bez_path);
    }
    fn handle_post_render(&mut self, _rtc: &mut RenderTreeContext<R>, rc: &mut R) {
        //pop the clipping context from the stack
        rc.restore().unwrap();
    }

    fn handle_post_mount(&mut self, rtc: &mut RenderTreeContext<R>) {
        let id_chain = rtc.get_id_chain(self.instance_id);
        (*rtc.engine.runtime).borrow_mut().enqueue_native_message(
            pax_message::NativeMessage::FrameCreate(AnyCreatePatch {
                id_chain: id_chain.clone(),
                clipping_ids: vec![]
            })
        );

        (*rtc.runtime).borrow_mut().push_clipping_stack_id(id_chain);
    }

    fn handle_pre_unmount(&mut self, rtc: &mut RenderTreeContext<R>) {
        let id_chain = rtc.get_id_chain(self.instance_id);
        (*rtc.engine.runtime).borrow_mut().enqueue_native_message(
            pax_message::NativeMessage::FrameDelete(id_chain)
        );

        (*rtc.runtime).borrow_mut().pop_clipping_stack_id();
    }

}
