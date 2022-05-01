use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
use pax_core::{ComponentInstance, PropertyExpression, RenderNodePtrList, RenderTreeContext, ExpressionContext, PaxEngine, RenderNode, InstanceRegistry, HandlerRegistry, InstantiationArgs, ConditionalInstance, SlotInstance, StackFrame};
use pax_core::pax_properties_coproduct::{PropertiesCoproduct, TypesCoproduct};
use pax_core::repeat::{RepeatInstance};
use piet_common::RenderContext;

use pax_runtime_api::{ArgsCoproduct, PropertyInstance, PropertyLiteral, Size2D, Transform2D};

//generate dependencies, pointing to userland cartridge (same logic as in PropertiesCoproduct)
use pax_example::pax_types::{Root};
use pax_example::pax_types::pax_std::primitives::{Rectangle, Group};
use pax_example::pax_types::pax_std::types::{Color, Stroke, Size, SpreadCellProperties};
use pax_example::pax_types::pax_std::components::Spread;

//dependency paths below come from pax_primitive macro, where these crate+module paths are passed as parameters:
use pax_std_primitives::{RectangleInstance, GroupInstance, FrameInstance};


pub fn instantiate_expression_table<R: 'static + RenderContext>() -> HashMap<String, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> {
    let mut vtable: HashMap<String, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> = HashMap::new();

    //literal string IDs to be generated by compiler, probably better as ints

    //note that type coercion should happen here, too:
    //(must know symbol name as well as source & destination types)
    //(compiler can keep a dict of operand types)

    vtable.insert("a".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!() };

        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;
        let i = i as f64;

        // datum

        //#[allow(non_snake_case)]
        // let self__DOT__rotation = if let PropertiesCoproduct::Root(p) = &*(*(*(*ec.stack_frame).borrow().get_scope()).borrow().properties).borrow() {
        //     *p.current_rotation.get()
        // } else { unreachable!() };

        return TypesCoproduct::Transform2D(
            Transform2D::align(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform2D::anchor(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform2D::rotate(__AT__frames_elapsed * i / 100.0) *
            Transform2D::translate(i * 10.0, i * 10.0) *
            Transform2D::rotate(__AT__frames_elapsed / 50.0)
        )
        // } else {unreachable!()};

    }));


    vtable.insert("c".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;

        //TODO: how to determine (for Expression codegen) that StrokeProperties is compound and requires
        //      wrapping in PropertyLiteral values?
        TypesCoproduct::Stroke(
            Stroke {
                color: Box::new(PropertyLiteral::new(Color::hlca((__AT__frames_elapsed as isize % 360) as f64, 100.0,100.0,1.0) )),
                width: Box::new(PropertyLiteral::new(45.0)),
            }
        )
    }));

    //this expression handles re-packing `data_list` for
    //`@for (elem, i) in computed_layout_spec {`
    vtable.insert("f".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {

        //get computed_layout_spec, which is a Vec<Rc<SpreadCellProperties>>
        //map (enumerate) its elements as elem into (elem, i),

        // let computed_layout_spec = : Vec<SpreadCellProperties>


        //#[allow(non_snake_case)]
        // let computed_layout_spec = if let PropertiesCoproduct::Root(p) = &*(*(*(*ec.stack_frame).borrow().get_scope()).borrow().properties).borrow() {
        //     *p.current_rotation.get()
        // } else { unreachable!() };


        //note this unwrapping is nested inside the `if let`, rather than flatted into a single assignment.
        //This is necessary for the non-clonable `Vec` in this case, and might need/want to be applied across codegen
        //(nesting instead of implicit cloning, e.g. of primitive types)
        #[allow(non_snake_case)]
        if let PropertiesCoproduct::Spread(p) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let computed_layout_spec = p.computed_layout_spec.get();
            return TypesCoproduct::Vec_Rc_PropertiesCoproduct___(computed_layout_spec.iter().enumerate().map(|(i,e)|{
                let cloned = Rc::clone(e);
                //TODO: there should be a way to pull off this re-wrapping without cloning the data structure (below).  One option is to deal with raw refs to the datum (we
                //are guaranteed immutable reads for this data, after all.)
                let rewrapped = PropertiesCoproduct::SpreadCellProperties((*cloned).clone());
                Rc::new(rewrapped)
            }).collect());
        } else { unreachable!() };

    }));

    vtable.insert("g".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("alpha") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!("beta")};

        return TypesCoproduct::Transform2D(
            Transform2D::translate(datum_cast.x_px, datum_cast.y_px)
        )

    }));

    //Frame size x
    vtable.insert("h".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        // const STACK_FRAME_OFFSET : isize = 1;
        // let STARTING_FRAME = (*ec.stack_frame).borrow().nth_descendant(STACK_FRAME_OFFSET); //just gen `ec.stack_frame` if offset == 0
        // pax_runtime_api::log(&format!("h: {:?}", &*(*(*STARTING_FRAME).borrow().get_properties()).borrow()));
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("gamma") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!("epsilon")};
        // (*ec.engine.runtime).borrow().log(&format!("evaling layout width {}", datum_cast.width_px));
        return TypesCoproduct::Size(
            Size::Pixel(datum_cast.width_px)
        )
    }));

    //Frame size y
    vtable.insert("i".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("delta") };

        let datum_cast = if let PropertiesCoproduct::SpreadCellProperties(d)= &*datum {d} else {unreachable!()};

        return TypesCoproduct::Size(
            Size::Pixel(datum_cast.height_px)
        )
    }));

    //Frame index
    vtable.insert("j".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        let (datum, i) = if let PropertiesCoproduct::RepeatItem(datum, i) = &*(*(*ec.stack_frame).borrow().get_properties()).borrow() {
            let x = (*ec.engine).borrow();
            (Rc::clone(datum), *i)
        } else { unreachable!("epsilon") };

        return TypesCoproduct::usize(
            i
        );
    }));

    vtable.insert("k".to_string(), Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
        #[allow(non_snake_case)]

        //example of pulling property from parent
        // let this_frame = (*ec.stack_frame).borrow();
        // let an0_frame = this_frame.parent.as_ref().unwrap();
        // let an0_frame_borrowed = (**an0_frame).borrow();
        // let properties = &*(*an0_frame_borrowed.properties).borrow();
        //
        // let current_rotation = if let PropertiesCoproduct::Root(p) = properties {
        //     *p.current_rotation.get() + 1.4
        // } else { unreachable!() };
        const STACK_FRAME_OFFSET : isize = 2;
        let SCOPED_STACK_FRAME = (*ec.stack_frame).borrow().nth_descendant(STACK_FRAME_OFFSET); //just gen `ec.stack_frame` if offset == 0

        let properties = SCOPED_STACK_FRAME.deref().borrow().get_properties();
        let properties = &*(*properties).borrow();

        // pax_runtime_api::log(&format!("Properties: {:?}", properties));
        let current_rotation = if let PropertiesCoproduct::Root(p) = properties {
            // pax_runtime_api::log( &format!("current_rotation: {}", p.current_rotation.get()));
            *p.current_rotation.get() as f64
        } else { unreachable!("zeta") };

        TypesCoproduct::Transform2D(
            Transform2D::anchor(Size::Percent(50.0), Size::Percent(50.0))
                * Transform2D::align(Size::Percent(50.0), Size::Percent(50.0))
                * Transform2D::rotate(current_rotation)
        )
    }));

    vtable
}

pub fn instantiate_root_component<R: 'static + RenderContext>(instance_registry: Rc<RefCell<InstanceRegistry<R>>>) -> Rc<RefCell<ComponentInstance<R>>> {
    //Root
    ComponentInstance::instantiate(
        InstantiationArgs{
            properties: PropertiesCoproduct::Root(Root {
                //these values are code-genned by pax-compiler.  If not provided, pax-compiler
                //can inject Default::default.  If the rust compiler throws an error,
                //that is the user's responsibility.
                num_clicks: Default::default(),
                current_rotation: Box::new(PropertyLiteral::new(0.0)),
            }),
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![],
                pre_render_handlers: vec![
                    |properties,args|{
                        let properties = &mut *properties.as_ref().borrow_mut();
                        let properties = if let PropertiesCoproduct::Root(p) = properties {p} else {unreachable!()};
                        Root::handle_pre_render(properties, args);
                    }
                ]
            }))),
            instance_registry: Rc::clone(&instance_registry),
            transform: Transform2D::default_wrapped(),
            size: None,
            children: None,
            component_template: Some(Rc::new(RefCell::new(vec![

                //Spread
                ComponentInstance::instantiate(
                    InstantiationArgs {
                        properties: PropertiesCoproduct::Spread(Spread {
                            computed_layout_spec: Default::default(),
                            direction: Default::default(),
                            cell_count: Box::new(PropertyLiteral::new(10)),
                            gutter_width: Box::new(PropertyLiteral::new(Size::Pixel(5.0))),
                            overrides_cell_size: Default::default(),
                            overrides_gutter_size: Default::default(),
                        }),
                        handler_registry: Some(Rc::new(RefCell::new(
                            HandlerRegistry {
                                click_handlers: vec![],
                                pre_render_handlers: vec![
                                    |properties,args|{
                                        let properties = &mut *properties.as_ref().borrow_mut();
                                        let properties = if let PropertiesCoproduct::Spread(p) = properties {p} else {unreachable!()};
                                        Spread::handle_pre_render(properties, args);
                                    }
                                ],
                            }
                        ))),
                        instance_registry: Rc::clone(&instance_registry),
                        transform: Transform2D::default_wrapped(),
                        size: Some([Box::new(PropertyLiteral::new(Size::Percent(100.0))), Box::new(PropertyLiteral::new(Size::Percent(100.0)))]),
                        children: Some(Rc::new(RefCell::new(vec![
                            RectangleInstance::instantiate(InstantiationArgs{
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new( pax_example::pax_types::pax_std::types::Stroke{
                                        color: Box::new(PropertyLiteral::new(Color::rgba(0.0,0.0,0.0,0.0))),
                                        width: Box::new(PropertyLiteral::new(0.0)),
                                    } )),
                                    fill: Box::new(PropertyLiteral::new(Color::rgba(0.0, 1.0, 1.0, 1.0)))
                                }),
                                handler_registry: None,
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Transform2D::default_wrapped(),
                                size: Some([PropertyLiteral::new(Size::Percent(100.0)).into(),PropertyLiteral::new(Size::Percent(100.0)).into()]),
                                children: None,
                                component_template: None,
                                slot_index: None,
                                should_skip_adoption: false,
                                repeat_data_list: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                            RepeatInstance::instantiate(InstantiationArgs {
                                properties: PropertiesCoproduct::None,
                                handler_registry: None,
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Transform2D::default_wrapped(),
                                size: None,
                                children: Some(Rc::new(RefCell::new( vec![
                                    RectangleInstance::instantiate(InstantiationArgs{
                                        properties: PropertiesCoproduct::Rectangle(Rectangle {
                                            stroke: Box::new(PropertyLiteral::new( pax_example::pax_types::pax_std::types::Stroke{
                                                color: Box::new(PropertyLiteral::new(Color::rgba(0.0,0.0,0.0,0.0))),
                                                width: Box::new(PropertyLiteral::new(0.0)),
                                            })),
                                            fill: Box::new(PropertyLiteral::new(Color::rgba(1.0, 0.45, 0.25, 1.0)))
                                        }),
                                        handler_registry: None,
                                        instance_registry: Rc::clone(&instance_registry),
                                        transform: Transform2D::default_wrapped(),
                                        size: Some([PropertyLiteral::new(Size::Percent(100.0)).into(),PropertyLiteral::new(Size::Percent(100.0)).into()]),
                                        children: None,
                                        component_template: None,
                                        slot_index: None,
                                        should_skip_adoption: false,
                                        repeat_data_list: None,
                                        conditional_boolean_expression: None,
                                        compute_properties_fn: None
                                        })
                                    ]
                                ))),
                                component_template: None,
                                should_skip_adoption: false,
                                slot_index: None,
                                repeat_data_list: Some(Box::new(PropertyLiteral::new((0..8).into_iter().map(|i|{
                                    Rc::new(PropertiesCoproduct::isize(i))
                                }).collect()))),
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                            RectangleInstance::instantiate(InstantiationArgs{
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new( pax_example::pax_types::pax_std::types::Stroke{
                                        color: Box::new(PropertyLiteral::new(Color::rgba(0.0,0.0,0.0,0.0))),
                                        width: Box::new(PropertyLiteral::new(0.0)),
                                    } )),
                                    fill: Box::new(PropertyLiteral::new(Color::rgba(1.0, 1.0, 0.0, 1.0)))
                                }),
                                handler_registry: None,
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new("k".to_string()))),
                                size: Some([PropertyLiteral::new(Size::Percent(100.0)).into(),PropertyLiteral::new(Size::Percent(100.0)).into()]),
                                children: None,
                                component_template: None,
                                slot_index: None,
                                should_skip_adoption: false,
                                repeat_data_list: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None
                            }),
                        ]))),
                        component_template: Some(Rc::new(RefCell::new(
                            vec![
                                RepeatInstance::instantiate(InstantiationArgs {
                                    properties: PropertiesCoproduct::None,
                                    handler_registry: None,
                                    instance_registry: Rc::clone(&instance_registry),
                                    transform: Transform2D::default_wrapped(),
                                    size: None,
                                    component_template: None,
                                    children: Some(Rc::new(RefCell::new(vec![
                                        FrameInstance::instantiate(InstantiationArgs{
                                            properties: PropertiesCoproduct::None,
                                            handler_registry: None,
                                            instance_registry: Rc::clone(&instance_registry),
                                            transform: Rc::new(RefCell::new(PropertyExpression::new("g".to_string()))),
                                            size: Some([
                                                Box::new(PropertyExpression::new("h".to_string())),
                                                Box::new(PropertyExpression::new("i".to_string())),
                                            ]),
                                            children: Some(Rc::new(RefCell::new(vec![
                                                SlotInstance::instantiate(InstantiationArgs {
                                                    properties: PropertiesCoproduct::None,
                                                    handler_registry: None,
                                                    instance_registry: Rc::clone(&instance_registry),
                                                    transform: Transform2D::default_wrapped(),
                                                    size: Some([PropertyLiteral::new(Size::Percent(100.0)).into(),PropertyLiteral::new(Size::Percent(100.0)).into()]),
                                                    children: None,
                                                    component_template: None,
                                                    should_skip_adoption: false,
                                                    slot_index: Some(Box::new(PropertyExpression::new("j".to_string()))),
                                                    repeat_data_list: None,
                                                    conditional_boolean_expression: None,
                                                    compute_properties_fn: None
                                                }),
                                            ]))),
                                            component_template: None,
                                            should_skip_adoption: false,
                                            slot_index: None,
                                            repeat_data_list: None,
                                            conditional_boolean_expression: None,
                                            compute_properties_fn: None
                                        }),
                                    ]))),
                                    slot_index: None,
                                    repeat_data_list: Some(Box::new(PropertyExpression::new("f".to_string()))),
                                    conditional_boolean_expression: None,
                                    compute_properties_fn: None,
                                    should_skip_adoption: false
                                }),
                            ]
                        ))),
                        should_skip_adoption: false,
                        slot_index: None,
                        repeat_data_list: None,
                        conditional_boolean_expression: None,
                        compute_properties_fn: Some(Box::new(|properties, rtc|{
                            let properties = &mut *properties.as_ref().borrow_mut();
                            let properties = if let PropertiesCoproduct::Spread(p) = properties {p} else {unreachable!()};

                            // if let Some(new_value) = rtc.get_eased_value(properties.direction._get_transition_manager()) {
                            //     properties.direction.set(new_value);
                            // }else
                            if let Some(new_value) = rtc.compute_vtable_value(properties.direction._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::SpreadDirection(v) = new_value { v } else { unreachable!() };
                                properties.direction.set(new_value);
                            }

                            if let Some(new_value) = rtc.compute_vtable_value(properties.cell_count._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::usize(v) = new_value { v } else { unreachable!() };
                                properties.cell_count.set(new_value);
                            }

                            if let Some(new_value) = rtc.compute_vtable_value(properties.gutter_width._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Size(v) = new_value { v } else { unreachable!() };
                                properties.gutter_width.set(new_value);
                            }

                            if let Some(new_value) = rtc.compute_vtable_value(properties.overrides_cell_size._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Vec_LPAREN_usize_COMMA_Size_RPAREN(v) = new_value { v } else { unreachable!() };
                                properties.overrides_cell_size.set(new_value);
                            }

                            if let Some(new_value) = rtc.compute_vtable_value(properties.overrides_gutter_size._get_vtable_id()) {
                                let new_value = if let TypesCoproduct::Vec_LPAREN_usize_COMMA_Size_RPAREN(v) = new_value { v } else { unreachable!() };
                                properties.overrides_gutter_size.set(new_value);
                            }

                        }))
                    }
                ),
                //End Spread

            ]))),
            should_skip_adoption: false,
            slot_index: None,
            repeat_data_list: None,
            conditional_boolean_expression: None,
            compute_properties_fn: Some(Box::new(|properties, rtc|{
                let properties = &mut *properties.as_ref().borrow_mut();
                let properties = if let PropertiesCoproduct::Root(p) = properties {p} else {unreachable!()};

                if let Some(new_value) = rtc.compute_eased_value(properties.current_rotation._get_transition_manager()) {
                    properties.current_rotation.set(new_value);
                }else if let Some(new_current_rotation) = rtc.compute_vtable_value(properties.current_rotation._get_vtable_id()) {
                    let new_value = if let TypesCoproduct::f64(v) = new_current_rotation { v } else { unreachable!() };
                    properties.current_rotation.set(new_value);
                }

                if let Some(new_num_clicks) = rtc.compute_vtable_value(properties.num_clicks._get_vtable_id()) {
                    let new_value = if let TypesCoproduct::isize(v) = new_num_clicks { v } else { unreachable!() };
                    properties.num_clicks.set(new_value);
                }

                // if let Some(new_deeper_struct) = rtc.get_computed_value(properties.deeper_struct._get_vtable_id()) {
                //     let new_value = if let TypesCoproduct::DeeperStruct(v) = new_deeper_struct { v } else { unreachable!() };
                //     properties.deeper_struct.set(new_value);
                // }
            }))
        }
    )
}