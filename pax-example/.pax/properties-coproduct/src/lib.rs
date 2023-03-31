use std::cell::RefCell;
use std::rc::Rc;

pub enum PropertiesCoproduct {
    None,
    RepeatList(Vec<Rc<RefCell<PropertiesCoproduct>>>),
    RepeatItem(Rc<PropertiesCoproduct>, usize),
    usize(usize), //used by Repeat with numeric ranges, like `for i in 0..5`
    
    Ellipse(pax_example::pax_reexports::pax_std::primitives::Ellipse),
    
    HelloRGB(pax_example::pax_reexports::HelloRGB),
    
    Path(pax_example::pax_reexports::pax_std::primitives::Path),
    
    Rectangle(pax_example::pax_reexports::pax_std::primitives::Rectangle),
    
    Text(pax_example::pax_reexports::pax_std::primitives::Text),
    
}

//used namely for return types of expressions — may have other purposes
pub enum TypesCoproduct {
    
    Size(pax_runtime_api::Size),
    
    Size2D(pax_runtime_api::Size2D),
    
    SizePixels(pax_runtime_api::SizePixels),
    
    String(String),
    
    Transform2D(pax_runtime_api::Transform2D),
    
    VecLABR__pax_stdCOCOtypesCOCOPathSegmentRABR(Vec<pax_example::pax_reexports::pax_std::types::PathSegment>),
    
    Vec_Rc_PropertiesCoproduct___(std::vec::Vec<std::rc::Rc<PropertiesCoproduct>>),
    
    __f64(pax_example::pax_reexports::f64),
    
    __pax_stdCOCOtypesCOCOColor(pax_example::pax_reexports::pax_std::types::Color),
    
    __pax_stdCOCOtypesCOCOFont(pax_example::pax_reexports::pax_std::types::Font),
    
    __pax_stdCOCOtypesCOCOStroke(pax_example::pax_reexports::pax_std::types::Stroke),
    
    __stdCOCOstringCOCOString(pax_example::pax_reexports::std::string::String),
    
    bool(bool),
    
    f64(f64),
    
    isize(isize),
    
    usize(usize),
    
}
