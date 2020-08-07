use std::os::raw;

pub type GLenum = raw::c_uint;

pub struct Nurbs {
    _inner: *mut glu_sys::GLUnurbs,
}

pub struct Tesselator {
    _inner: *mut glu_sys::GLUtesselator,
}

pub struct Quadric {
    _inner: *mut glu_sys::GLUquadric,
}

pub fn BeginCurve(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluBeginCurve(nurb._inner) }
}

pub fn BeginPolygon(tess: &mut Tesselator) {
    unsafe { glu_sys::gluBeginPolygon(tess._inner) }
}

pub fn BeginSurface(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluBeginSurface(nurb._inner) }
}

pub fn BeginTrim(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluBeginTrim(nurb._inner) }
}

pub fn Build1DMipmapLevels(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    format: GLenum,
    type_: GLenum,
    level: i32,
    base: i32,
    max: i32,
    data: *const raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluBuild1DMipmapLevels(
            target,
            internalFormat,
            width,
            format,
            type_,
            level,
            base,
            max,
            data,
        )
    }
}

pub fn Build1DMipmaps(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    format: GLenum,
    type_: GLenum,
    data: *const raw::c_void,
) -> i32 {
    unsafe { glu_sys::gluBuild1DMipmaps(target, internalFormat, width, format, type_, data) }
}

pub fn Build2DMipmapLevels(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    height: i32,
    format: GLenum,
    type_: GLenum,
    level: i32,
    base: i32,
    max: i32,
    data: *const raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluBuild2DMipmapLevels(
            target,
            internalFormat,
            width,
            height,
            format,
            type_,
            level,
            base,
            max,
            data,
        )
    }
}

pub fn Build2DMipmaps(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    height: i32,
    format: GLenum,
    type_: GLenum,
    data: *const raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluBuild2DMipmaps(target, internalFormat, width, height, format, type_, data)
    }
}

pub fn Build3DMipmapLevels(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: GLenum,
    type_: GLenum,
    level: i32,
    base: i32,
    max: i32,
    data: *const raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluBuild3DMipmapLevels(
            target,
            internalFormat,
            width,
            height,
            depth,
            format,
            type_,
            level,
            base,
            max,
            data,
        )
    }
}

pub fn Build3DMipmaps(
    target: GLenum,
    internalFormat: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: GLenum,
    type_: GLenum,
    data: *const raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluBuild3DMipmaps(
            target,
            internalFormat,
            width,
            height,
            depth,
            format,
            type_,
            data,
        )
    }
}

pub fn CheckExtension(extName: &str, extString: &str) -> bool {
    unsafe {
        let extName = std::ffi::CString::new(extName).unwrap();
        let extString = std::ffi::CString::new(extString).unwrap();
        glu_sys::gluCheckExtension(
            extName.as_ptr() as *const raw::c_uchar,
            extString.as_ptr() as *const raw::c_uchar,
        ) != 0
    }
}

pub fn Cylinder(quad: &mut Quadric, base: f64, top: f64, height: f64, slices: i32, stacks: i32) {
    unsafe { glu_sys::gluCylinder(quad._inner, base, top, height, slices, stacks) }
}

pub fn DeleteNurbsRenderer(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluDeleteNurbsRenderer(nurb._inner) }
}

pub fn DeleteQuadric(quad: &mut Quadric) {
    unsafe { glu_sys::gluDeleteQuadric(quad._inner) }
}

pub fn DeleteTess(tess: &mut Tesselator) {
    unsafe { glu_sys::gluDeleteTess(tess._inner) }
}

pub fn Disk(quad: &mut Quadric, inner: f64, outer: f64, slices: i32, loops: i32) {
    unsafe { glu_sys::gluDisk(quad._inner, inner, outer, slices, loops) }
}

pub fn EndCurve(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluEndCurve(nurb._inner) }
}

pub fn EndPolygon(tess: &mut Tesselator) {
    unsafe { glu_sys::gluEndPolygon(tess._inner) }
}

pub fn EndSurface(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluEndSurface(nurb._inner) }
}

pub fn EndTrim(nurb: &mut Nurbs) {
    unsafe { glu_sys::gluEndTrim(nurb._inner) }
}

pub fn ErrorString(error: GLenum) -> Option<String> {
    unsafe {
        let ptr = glu_sys::gluErrorString(error);
        if ptr.is_null() {
            None
        } else {
            Some(
                std::ffi::CStr::from_ptr(ptr as *const raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }
}

pub fn GetNurbsProperty(nurb: &mut Nurbs, property: GLenum, data: &mut [f32]) {
    unsafe {
        glu_sys::gluGetNurbsProperty(nurb._inner, property, data.as_mut_ptr());
    }
}

pub fn GetString(name: GLenum) -> Option<String> {
    unsafe {
        let ptr = glu_sys::gluGetString(name);
        if ptr.is_null() {
            None
        } else {
            Some(
                std::ffi::CStr::from_ptr(ptr as *const raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }
}

pub fn GetTessProperty(tess: &mut Tesselator, which: GLenum, data: &mut [f64]) {
    unsafe {
        glu_sys::gluGetTessProperty(tess._inner, which, data.as_mut_ptr());
    }
}

pub fn LoadSamplingMatrices(nurb: &mut Nurbs, model: &[f32], perspective: &[f32], view: &[i32]) {
    unsafe {
        glu_sys::gluLoadSamplingMatrices(
            nurb._inner,
            model.as_ptr(),
            perspective.as_ptr(),
            view.as_ptr(),
        )
    }
}

pub fn LookAt(
    eyeX: f64,
    eyeY: f64,
    eyeZ: f64,
    centerX: f64,
    centerY: f64,
    centerZ: f64,
    upX: f64,
    upY: f64,
    upZ: f64,
) {
    unsafe { glu_sys::gluLookAt(eyeX, eyeY, eyeZ, centerX, centerY, centerZ, upX, upY, upZ) }
}

pub fn NewNurbsRenderer() -> Nurbs {
    unsafe {
        let ptr = glu_sys::gluNewNurbsRenderer();
        assert!(!ptr.is_null());
        Nurbs { _inner: ptr }
    }
}

pub fn NewQuadric() -> Quadric {
    unsafe {
        let ptr = glu_sys::gluNewQuadric();
        assert!(!ptr.is_null());
        Quadric { _inner: ptr }
    }
}

pub fn NewTess() -> Tesselator {
    unsafe {
        let ptr = glu_sys::gluNewTess();
        assert!(!ptr.is_null());
        Tesselator { _inner: ptr }
    }
}

pub fn NextContour(tess: &mut Tesselator, type_: GLenum) {
    unsafe { glu_sys::gluNextContour(tess._inner, type_) }
}

pub fn NurbsCallback(nurb: &mut Nurbs, which: GLenum, CallBackFunc: Option<fn()>) {
    unsafe { glu_sys::gluNurbsCallback(nurb._inner, which, std::mem::transmute(CallBackFunc)) }
}

pub fn NurbsCallbackData(nurb: &mut Nurbs, userData: *mut raw::c_void) {
    unsafe { glu_sys::gluNurbsCallbackData(nurb._inner, userData) }
}

pub fn NurbsCallbackDataEXT(nurb: &mut Nurbs, userData: *mut raw::c_void) {
    unsafe { glu_sys::gluNurbsCallbackDataEXT(nurb._inner, userData) }
}

pub fn NurbsCurve(
    nurb: &mut Nurbs,
    knots: &mut [f32],
    stride: i32,
    control: &mut [f32],
    order: i32,
    type_: GLenum,
) {
    unsafe {
        glu_sys::gluNurbsCurve(
            nurb._inner,
            knots.len() as i32,
            knots.as_mut_ptr(),
            stride,
            control.as_mut_ptr(),
            order,
            type_,
        )
    }
}

pub fn NurbsProperty(nurb: &mut Nurbs, property: GLenum, value: f32) {
    unsafe { glu_sys::gluNurbsProperty(nurb._inner, property, value) }
}

pub fn NurbsSurface(
    nurb: &mut Nurbs,
    sKnots: &mut [f32],
    tKnots: &mut [f32],
    sStride: i32,
    tStride: i32,
    control: &mut [f32],
    sOrder: i32,
    tOrder: i32,
    type_: GLenum,
) {
    unsafe {
        glu_sys::gluNurbsSurface(
            nurb._inner,
            sKnots.len() as i32,
            sKnots.as_mut_ptr(),
            tKnots.len() as i32,
            tKnots.as_mut_ptr(),
            sStride,
            tStride,
            control.as_mut_ptr(),
            sOrder,
            tOrder,
            type_,
        )
    }
}

pub fn Ortho2D(left: f64, right: f64, bottom: f64, top: f64) {
    unsafe { glu_sys::gluOrtho2D(left, right, bottom, top) }
}

pub fn PartialDisk(
    quad: &mut Quadric,
    inner: f64,
    outer: f64,
    slices: i32,
    loops: i32,
    start: f64,
    sweep: f64,
) {
    unsafe { glu_sys::gluPartialDisk(quad._inner, inner, outer, slices, loops, start, sweep) }
}

pub fn Perspective(fovy: f64, aspect: f64, zNear: f64, zFar: f64) {
    unsafe { glu_sys::gluPerspective(fovy, aspect, zNear, zFar) }
}

pub fn PickMatrix(x: f64, y: f64, delX: f64, delY: f64, viewport: &mut [i32]) {
    unsafe {
        glu_sys::gluPickMatrix(x, y, delX, delY, viewport.as_mut_ptr());
    }
}

pub fn Project(
    objX: f64,
    objY: f64,
    objZ: f64,
    model: &[f64],
    proj: &[f64],
    view: &[i32],
    winX: &mut [f64],
    winY: &mut [f64],
    winZ: &mut [f64],
) -> i32 {
    unsafe {
        glu_sys::gluProject(
            objX,
            objY,
            objZ,
            model.as_ptr(),
            proj.as_ptr(),
            view.as_ptr(),
            winX.as_mut_ptr(),
            winY.as_mut_ptr(),
            winZ.as_mut_ptr(),
        )
    }
}

pub fn PwlCurve(nurb: &mut Nurbs, data: &mut [f32], stride: i32, type_: GLenum) {
    unsafe {
        glu_sys::gluPwlCurve(
            nurb._inner,
            data.len() as i32,
            data.as_mut_ptr(),
            stride,
            type_,
        )
    }
}

pub fn QuadricCallback(quad: &mut Quadric, which: GLenum, CallBackFunc: Option<fn()>) {
    unsafe { glu_sys::gluQuadricCallback(quad._inner, which, std::mem::transmute(CallBackFunc)) }
}

pub fn QuadricDrawStyle(quad: &mut Quadric, draw: GLenum) {
    unsafe { glu_sys::gluQuadricDrawStyle(quad._inner, draw) }
}

pub fn QuadricNormals(quad: &mut Quadric, normal: GLenum) {
    unsafe { glu_sys::gluQuadricNormals(quad._inner, normal) }
}

pub fn QuadricOrientation(quad: &mut Quadric, orientation: GLenum) {
    unsafe { glu_sys::gluQuadricOrientation(quad._inner, orientation) }
}

pub fn QuadricTexture(quad: &mut Quadric, texture: bool) {
    unsafe { glu_sys::gluQuadricTexture(quad._inner, texture as u8) }
}

pub fn ScaleImage(
    format: GLenum,
    wIn: i32,
    hIn: i32,
    typeIn: GLenum,
    dataIn: *const raw::c_void,
    wOut: i32,
    hOut: i32,
    typeOut: GLenum,
    dataOut: *mut raw::c_void,
) -> i32 {
    unsafe {
        glu_sys::gluScaleImage(
            format, wIn, hIn, typeIn, dataIn, wOut, hOut, typeOut, dataOut,
        )
    }
}

pub fn Sphere(quad: &mut Quadric, radius: f64, slices: i32, stacks: i32) {
    unsafe { glu_sys::gluSphere(quad._inner, radius, slices, stacks) }
}

pub fn TessBeginContour(tess: &mut Tesselator) {
    unsafe { glu_sys::gluTessBeginContour(tess._inner) }
}

pub fn TessBeginPolygon(tess: &mut Tesselator, data: *mut raw::c_void) {
    unsafe { glu_sys::gluTessBeginPolygon(tess._inner, data) }
}

pub fn TessCallback(tess: &mut Tesselator, which: GLenum, CallBackFunc: Option<fn()>) {
    unsafe { glu_sys::gluTessCallback(tess._inner, which, std::mem::transmute(CallBackFunc)) }
}

pub fn TessEndContour(tess: &mut Tesselator) {
    unsafe { glu_sys::gluTessEndContour(tess._inner) }
}

pub fn TessEndPolygon(tess: &mut Tesselator) {
    unsafe { glu_sys::gluTessEndPolygon(tess._inner) }
}

pub fn TessNormal(tess: &mut Tesselator, valueX: f64, valueY: f64, valueZ: f64) {
    unsafe { glu_sys::gluTessNormal(tess._inner, valueX, valueY, valueZ) }
}

pub fn TessProperty(tess: &mut Tesselator, which: GLenum, data: f64) {
    unsafe { glu_sys::gluTessProperty(tess._inner, which, data) }
}

pub fn TessVertex(tess: &mut Tesselator, location: &mut [f64], data: *mut raw::c_void) {
    unsafe { glu_sys::gluTessVertex(tess._inner, location.as_mut_ptr(), data) }
}

pub fn UnProject(
    winX: f64,
    winY: f64,
    winZ: f64,
    model: &[f64],
    proj: &[f64],
    view: &[i32],
    objX: &mut [f64],
    objY: &mut [f64],
    objZ: &mut [f64],
) -> i32 {
    unsafe {
        glu_sys::gluUnProject(
            winX,
            winY,
            winZ,
            model.as_ptr(),
            proj.as_ptr(),
            view.as_ptr(),
            objX.as_mut_ptr(),
            objY.as_mut_ptr(),
            objZ.as_mut_ptr(),
        )
    }
}

pub fn UnProject4(
    winX: f64,
    winY: f64,
    winZ: f64,
    clipW: f64,
    model: &[f64],
    proj: &[f64],
    view: &[i32],
    nearVal: f64,
    farVal: f64,
    objX: &mut [f64],
    objY: &mut [f64],
    objZ: &mut [f64],
    objW: &mut [f64],
) -> i32 {
    unsafe {
        glu_sys::gluUnProject4(
            winX,
            winY,
            winZ,
            clipW,
            model.as_ptr(),
            proj.as_ptr(),
            view.as_ptr(),
            nearVal,
            farVal,
            objX.as_mut_ptr(),
            objY.as_mut_ptr(),
            objZ.as_mut_ptr(),
            objW.as_mut_ptr(),
        )
    }
}
