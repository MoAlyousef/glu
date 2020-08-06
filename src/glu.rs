// use std::os::raw;

// pub type GLenum = raw::c_uint;

// pub struct Nurbs {
//     _inner: *mut glu_sys::GLUnurbs,
// }

// pub struct Tesselator {
//     _inner: *mut glu_sys::GLUtesselator,
// }

// pub struct Quadric {
//     _inner: *mut glu_sys::GLUquadric,
// }


// pub fn gluBeginCurve(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluBeginCurve(nurb._inner)
//     }
// }


// pub fn gluBeginPolygon(tess: &mut Tesselator) {
//     unsafe {
//         glu_sys::gluBeginPolygon(tess._inner)
//     }
// }


// pub fn gluBeginSurface(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluBeginSurface(nurb._inner)
//     }
// }


// pub fn gluBeginTrim(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluBeginTrim(nurb._inner)
//     }
// }


// pub fn gluBuild1DMipmapLevels(target: GLenum, internalFormat: i32, width: i32, format: GLenum, type_: GLenum, level: i32, base: i32, max: i32, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild1DMipmapLevels(target, internalFormat, width, format, type_, level, base, max, data)
//     }
// }


// pub fn gluBuild1DMipmaps(target: GLenum, internalFormat: i32, width: i32, format: GLenum, type_: GLenum, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild1DMipmaps(target, internalFormat, width, format, type_, data)
//     }
// }


// pub fn gluBuild2DMipmapLevels(target: GLenum, internalFormat: i32, width: i32, height: i32, format: GLenum, type_: GLenum, level: i32, base: i32, max: i32, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild2DMipmapLevels(target, internalFormat, width, height, format, type_, level, base, max, data)
//     }
// }


// pub fn gluBuild2DMipmaps(target: GLenum, internalFormat: i32, width: i32, height: i32, format: GLenum, type_: GLenum, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild2DMipmaps(target, internalFormat, width, height, format, type_, data)
//     }
// }


// pub fn gluBuild3DMipmapLevels(target: GLenum, internalFormat: i32, width: i32, height: i32, depth: i32, format: GLenum, type_: GLenum, level: i32, base: i32, max: i32, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild3DMipmapLevels(target, internalFormat, width, height, depth, format, type_, level, base, max, data)
//     }
// }


// pub fn gluBuild3DMipmaps(target: GLenum, internalFormat: i32, width: i32, height: i32, depth: i32, format: GLenum, type_: GLenum, data: *const raw::c_void) -> i32 {
//     unsafe {
//         glu_sys::gluBuild3DMipmaps(target, internalFormat, width, height, depth, format, type_, data)
//     }
// }


// pub fn gluCheckExtension(extName: &str, extString: &str) -> bool {
//     unsafe {
//         let extName = std::ffi::CString::new(extName).unwrap();
//         let extString = std::ffi::CString::new(extString).unwrap();
//         glu_sys::gluCheckExtension(extName.as_ptr() as *const raw::c_uchar, extString.as_ptr() as *const raw::c_uchar) != 0
//     }
// }


// pub fn gluCylinder(quad: &mut Quadric, base: f64, top: f64, height: f64, slices: i32, stacks: i32) {
//     unsafe {
//         glu_sys::gluCylinder(quad._inner, base, top, height, slices, stacks)
//     }
// }


// pub fn gluDeleteNurbsRenderer(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluDeleteNurbsRenderer(nurb._inner)
//     }
// }


// pub fn gluDeleteQuadric(quad: &mut Quadric) {
//     unsafe {
//         glu_sys::gluDeleteQuadric(quad._inner)
//     }
// }


// pub fn gluDeleteTess(tess: &mut Tesselator) {
//     unsafe {
//         glu_sys::gluDeleteTess(tess._inner)
//     }
// }


// pub fn gluDisk(quad: &mut Quadric, inner: f64, outer: f64, slices: i32, loops: i32) {
//     unsafe {
//         glu_sys::gluDisk(quad._inner, inner, outer, slices, loops)
//     }
// }


// pub fn gluEndCurve(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluEndCurve(nurb._inner)
//     }
// }


// pub fn gluEndPolygon(tess: &mut Tesselator) {
//     unsafe {
//         glu_sys::gluEndPolygon(tess._inner)
//     }
// }


// pub fn gluEndSurface(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluEndSurface(nurb._inner)
//     }
// }


// pub fn gluEndTrim(nurb: &mut Nurbs) {
//     unsafe {
//         glu_sys::gluEndTrim(nurb._inner)
//     }
// }


// pub fn gluErrorString(error: GLenum) -> Option<String> {
//     unsafe {
//         let ptr = glu_sys::gluErrorString(error);
//         if ptr.is_null() {
//             None
//         } else {
//             Some(std::ffi::CStr::from_ptr(ptr as *const raw::c_char).to_string_lossy().to_string())
//         }
//     }
// }


// pub fn gluGetNurbsProperty(nurb: &mut Nurbs, property: GLenum) -> Vec<f32> {
//     unsafe {
//         let mut data = vec![];
//         glu_sys::gluGetNurbsProperty(nurb._inner, property, data.as_mut_ptr());
//         data
//     }
// }


// pub fn gluGetString(name: GLenum) -> Option<String> {
//     unsafe {
//         let ptr = glu_sys::gluGetString(name);
//         if ptr.is_null() {
//             None
//         } else {
//             Some(std::ffi::CStr::from_ptr(ptr as *const raw::c_char).to_string_lossy().to_string())
//         }
//     }
// }


// pub fn gluGetTessProperty(tess: &mut Tesselator, which: GLenum) -> Vec<f64> {
//     unsafe {
//         let mut data = vec![];
//         glu_sys::gluGetTessProperty(tess._inner, which, data.as_mut_ptr());
//         data
//     }
// }


// pub fn gluLoadSamplingMatrices(nurb: &mut Nurbs, model: &[f32], perspective: &[f32], view: &[i32]) {
//     unsafe {
//         glu_sys::gluLoadSamplingMatrices(nurb._inner, model.as_ptr(), perspective.as_ptr(), view.as_ptr())
//     }
// }


// pub fn gluLookAt(eyeX: f64, eyeY: f64, eyeZ: f64, centerX: f64, centerY: f64, centerZ: f64, upX: f64, upY: f64, upZ: f64) {
//     unsafe {
//         glu_sys::gluLookAt(eyeX, eyeY, eyeZ, centerX, centerY, centerZ, upX, upY, upZ)
//     }
// }


// pub fn gluNewNurbsRenderer() -> Nurbs {
//     unsafe {
//         let ptr = glu_sys::gluNewNurbsRenderer();
//         assert!(!ptr.is_null());
//         Nurbs { _inner: ptr }
//     }
// }


// pub fn gluNewQuadric() -> Quadric {
//     unsafe {
//         let ptr = glu_sys::gluNewQuadric();
//         assert!(!ptr.is_null());
//         Quadric { _inner: ptr }
//     }
// }


// pub fn gluNewTess() -> Tesselator {
//     unsafe {
//         let ptr = glu_sys::gluNewTess();
//         assert!(!ptr.is_null());
//         Tesselator { _inner: ptr }
//     }
// }


// pub fn gluNextContour(tess: &mut Tesselator, type_: GLenum) {
//     unsafe {
//         glu_sys::gluNextContour(tess._inner, type_)
//     }
// }


// pub fn gluNurbsCallback(nurb: &mut Nurbs, which: GLenum, CallBackFunc: Option<fn()>) {
//     unsafe {
//         glu_sys::gluNurbsCallback(nurb._inner, which, std::mem::transmute(CallBackFunc))
//     }
// }


// pub fn gluNurbsCallbackData(nurb: &mut Nurbs, userData: *mut raw::c_void) {
//     unsafe {
//         glu_sys::gluNurbsCallbackData(nurb._inner, userData)
//     }
// }


// pub fn gluNurbsCallbackDataEXT(nurb: &mut Nurbs, userData: *mut raw::c_void) {
//     unsafe {
//         glu_sys::gluNurbsCallbackDataEXT(nurb._inner, userData)
//     }
// }


// pub fn gluNurbsCurve(nurb: &mut Nurbs, knotCount: i32, knots: &[f32], stride: i32, control: &[f32], order: i32, type_: GLenum) {
//     unsafe {
//        glu_sys::gluNurbsCurve(nurb._inner, knotCount, knots.as_ptr(), stride, control.as_ptr(), order, type_)
//     }
// }


// pub fn gluNurbsProperty(nurb: &mut Nurbs, property: GLenum, value: f32) {
//     unsafe {
//         glu_sys::gluNurbsProperty(nurb._inner, property, value)
//     }
// }


// pub fn gluNurbsSurface(nurb: &mut Nurbs, sKnotCount: i32, sKnots: &[f32], tKnotCount: i32, tKnots: &[f32], sStride: i32, tStride: i32, control: &[f32], sOrder: i32, tOrder: i32, type_: GLenum) {
//     unsafe {
//         glu_sys::gluNurbsSurface(nurb._inner, sKnotCount, sKnots.as_ptr(), tKnotCount, tKnots.as_ptr(), sStride, tStride, control.as_ptr(), sOrder, tOrder, type_)
//     }
// }


// pub fn gluOrtho2D(left: f64, right: f64, bottom: f64, top: f64) {
//     unsafe {
//         glu_sys::gluOrtho2D(left, right, bottom, top)
//     }
// }


// pub fn gluPartialDisk(quad: &mut Quadric, inner: f64, outer: f64, slices: i32, loops: i32, start: f64, sweep: f64) {
//     unsafe {
//         glu_sys::gluPartialDisk(quad._inner, inner, outer, slices, loops, start, sweep)
//     }
// }


// pub fn gluPerspective(fovy: f64, aspect: f64, zNear: f64, zFar: f64) {
//     unsafe {
//         glu_sys::gluPerspective(fovy, aspect, zNear, zFar)
//     }
// }


// pub fn gluPickMatrix(x: f64, y: f64, delX: f64, delY: f64) -> Vec<i32> {
//     unsafe {
//         let mut viewport = vec![];
//         glu_sys::gluPickMatrix(x, y, delX, delY, viewport.as_mut_ptr());
//         viewport
//     }
// }


// // pub fn gluProject(objX: f64, objY: f64, objZ: f64, model: &[f64], proj: &[f64], view: &[i32], winX: &[f64], winY: &[f64], winZ: *mut f64) -> i32 {
// //     unsafe {
// //         glu_sys::gluProject(objX, objY, objZ, model.as_ptr(), proj.as_ptr(), view.as_ptr(), winX, winY, winZ)
// //     }
// // }


// // pub fn gluPwlCurve(nurb: *mut GLUnurbs, count: i32, data: *mut f32, stride: i32, type_: GLenum) {
// //     unsafe {
// //         glu_sys::gluPwlCurve(nurb: *mut GLUnurbs, count: i32, data: *mut f32, stride: i32, type_: GLenum)
// //     }
// // }


// // pub fn gluQuadricCallback(quad: *mut GLUquadric, which: GLenum, CallBackFunc: Option<fn()>) {
// //     unsafe {
// //         glu_sys::gluQuadricCallback(quad: *mut GLUquadric, which: GLenum, CallBackFunc: Option<fn()>)
// //     }
// // }


// // pub fn gluQuadricDrawStyle(quad: *mut GLUquadric, draw: GLenum) {
// //     unsafe {
// //         glu_sys::gluQuadricDrawStyle(quad: *mut GLUquadric, draw: GLenum)
// //     }
// // }


// // pub fn gluQuadricNormals(quad: *mut GLUquadric, normal: GLenum) {
// //     unsafe {
// //         glu_sys::gluQuadricNormals(quad: *mut GLUquadric, normal: GLenum)
// //     }
// // }


// // pub fn gluQuadricOrientation(quad: *mut GLUquadric, orientation: GLenum) {
// //     unsafe {
// //         glu_sys::gluQuadricOrientation(quad: *mut GLUquadric, orientation: GLenum)
// //     }
// // }


// // pub fn gluQuadricTexture(quad: *mut GLUquadric, texture: bool) {
// //     unsafe {
// //         glu_sys::gluQuadricTexture(quad: *mut GLUquadric, texture: bool)
// //     }
// // }


// // pub fn gluScaleImage(format: GLenum, wIn: i32, hIn: i32, typeIn: GLenum, dataIn: *const raw::c_void, wOut: i32, hOut: i32, typeOut: GLenum, dataOut: *mut raw::c_void) -> i32 {
// //     unsafe {
// //         glu_sys::: GLenum, dataOut: *mut raw::c_void) -> i32
// //     }
// // }


// // pub fn gluSphere(quad: *mut GLUquadric, radius: f64, slices: i32, stacks: i32) {
// //     unsafe {
// //         glu_sys::gluSphere(quad: *mut GLUquadric, radius: f64, slices: i32, stacks: i32)
// //     }
// // }


// // pub fn gluTessBeginContour(tess: *mut GLUtesselator) {
// //     unsafe {
// //         glu_sys::gluTessBeginContour(tess: *mut GLUtesselator)
// //     }
// // }


// // pub fn gluTessBeginPolygon(tess: *mut GLUtesselator, data: *mut raw::c_void) {
// //     unsafe {
// //         glu_sys::gluTessBeginPolygon(tess: *mut GLUtesselator, data: *mut raw::c_void)
// //     }
// // }


// // pub fn gluTessCallback(tess: *mut GLUtesselator, which: GLenum, CallBackFunc: Option<fn()>) {
// //     unsafe {
// //         glu_sys::gluTessCallback(tess: *mut GLUtesselator, which: GLenum, CallBackFunc: Option<fn()>)
// //     }
// // }


// // pub fn gluTessEndContour(tess: *mut GLUtesselator) {
// //     unsafe {
// //         glu_sys::gluTessEndContour(tess: *mut GLUtesselator)
// //     }
// // }


// // pub fn gluTessEndPolygon(tess: *mut GLUtesselator) {
// //     unsafe {
// //         glu_sys::gluTessEndPolygon(tess: *mut GLUtesselator)
// //     }
// // }


// // pub fn gluTessNormal(tess: *mut GLUtesselator, valueX: f64, valueY: f64, valueZ: f64) {
// //     unsafe {
// //         glu_sys::gluTessNormal(tess: *mut GLUtesselator, valueX: f64, valueY: f64, valueZ: f64)
// //     }
// // }


// // pub fn gluTessProperty(tess: *mut GLUtesselator, which: GLenum, data: f64) {
// //     unsafe {
// //         glu_sys::gluTessProperty(tess: *mut GLUtesselator, which: GLenum, data: f64)
// //     }
// // }


// // pub fn gluTessVertex(tess: *mut GLUtesselator, location: *mut f64, data: *mut raw::c_void) {
// //     unsafe {
// //         glu_sys::gluTessVertex(tess: *mut GLUtesselator, location: *mut f64, data: *mut raw::c_void)
// //     }
// // }


// // pub fn gluUnProject(winX: f64, winY: f64, winZ: f64, model: *const f64, proj: *const f64, view: *const i32, objX: *mut f64, objY: *mut f64, objZ: *mut f64) -> i32 {
// //     unsafe {
// //         glu_sys::4, objZ: *mut f64) -> i32
// //     }
// // }


// // pub fn gluUnProject4(winX: f64, winY: f64, winZ: f64, clipW: f64, model: *const f64, proj: *const f64, view: *const i32, nearVal: f64, farVal: f64, objX: *mut f64, objY: *mut f64, objZ: *mut f64, objW: *mut f64) -> i32 {
// //     unsafe {
// //         glu_sys::rVal: f64, objX: *mut f64, objY: *mut f64, objZ: *mut f64, objW: *mut f64) -> i32
// //     }
// // }