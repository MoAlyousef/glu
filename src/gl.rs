use std::os::raw;

pub type GLenum = raw::c_uint;

pub fn ClearIndex(c: f32) {
    unsafe { glu_sys::glClearIndex(c) }
}

pub fn ClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glu_sys::glClearColor(red, green, blue, alpha) }
}

pub fn Clear(mask: u32) {
    unsafe { glu_sys::glClear(mask) }
}

pub fn IndexMask(mask: u32) {
    unsafe { glu_sys::glIndexMask(mask) }
}

pub fn ColorMask(red: bool, green: bool, blue: bool, alpha: bool) {
    unsafe { glu_sys::glColorMask(red as u8, green as u8, blue as u8, alpha as u8) }
}

pub fn AlphaFunc(func: GLenum, ref_: f32) {
    unsafe { glu_sys::glAlphaFunc(func, ref_) }
}

pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum) {
    unsafe { glu_sys::glBlendFunc(sfactor, dfactor) }
}

pub fn LogicOp(opcode: GLenum) {
    unsafe { glu_sys::glLogicOp(opcode) }
}

pub fn CullFace(mode: GLenum) {
    unsafe { glu_sys::glCullFace(mode) }
}

pub fn FrontFace(mode: GLenum) {
    unsafe { glu_sys::glFrontFace(mode) }
}

pub fn PointSize(size: f32) {
    unsafe { glu_sys::glPointSize(size) }
}

pub fn LineWidth(width: f32) {
    unsafe { glu_sys::glLineWidth(width) }
}

pub fn LineStipple(factor: i32, pattern: u16) {
    unsafe { glu_sys::glLineStipple(factor, pattern) }
}

pub fn PolygonMode(face: GLenum, mode: GLenum) {
    unsafe { glu_sys::glPolygonMode(face, mode) }
}

pub fn PolygonOffset(factor: f32, units: f32) {
    unsafe { glu_sys::glPolygonOffset(factor, units) }
}

pub fn PolygonStipple(mask: &[u8; 32 * 32]) {
    unsafe { glu_sys::glPolygonStipple(mask.as_ptr()) }
}

pub fn GetPolygonStipple(mask: &mut [u8; 32 * 32]) {
    unsafe {
        glu_sys::glGetPolygonStipple(mask.as_mut_ptr());
    }
}

pub fn EdgeFlag(flag: bool) {
    unsafe { glu_sys::glEdgeFlag(flag as u8) }
}

pub fn Scissor(x: i32, y: i32, width: i32, height: i32) {
    unsafe { glu_sys::glScissor(x, y, width, height) }
}

pub fn ClipPlane(plane: u32, equation: &[f64; 4]) {
    unsafe { glu_sys::glClipPlane(plane, equation.as_ptr()) }
}

pub fn GetClipPlane(plane: u32, equation: &mut [f64; 4]) {
    unsafe {
        glu_sys::glGetClipPlane(plane, equation.as_mut_ptr());
    }
}

pub fn DrawBuffer(mode: GLenum) {
    unsafe { glu_sys::glDrawBuffer(mode) }
}

pub fn ReadBuffer(mode: GLenum) {
    unsafe { glu_sys::glReadBuffer(mode) }
}

pub fn Enable(cap: u32) {
    unsafe { glu_sys::glEnable(cap) }
}

pub fn Disable(cap: u32) {
    unsafe { glu_sys::glDisable(cap) }
}

pub fn IsEnabled(cap: u32) -> bool {
    unsafe { glu_sys::glIsEnabled(cap) != 0 }
}

pub fn EnableClientState(cap: u32) {
    unsafe { glu_sys::glEnableClientState(cap) }
}

pub fn DisableClientState(cap: u32) {
    unsafe { glu_sys::glDisableClientState(cap) }
}

pub fn GetBooleanv(pname: GLenum, params: &mut [bool]) {
    unsafe {
        glu_sys::glGetBooleanv(pname, params.as_mut_ptr() as *mut u8);
    }
}

pub fn GetDoublev(pname: GLenum, params: &mut [f64]) {
    unsafe {
        glu_sys::glGetDoublev(pname, params.as_mut_ptr());
    }
}

pub fn GetFloatv(pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetFloatv(pname, params.as_mut_ptr());
    }
}

pub fn GetIntegerv(pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetIntegerv(pname, params.as_mut_ptr());
    }
}

pub fn PushAttrib(mask: u32) {
    unsafe { glu_sys::glPushAttrib(mask) }
}

pub fn PopAttrib() {
    unsafe { glu_sys::glPopAttrib() }
}

pub fn PushClientAttrib(mask: u32) {
    unsafe { glu_sys::glPushClientAttrib(mask) }
}

pub fn PopClientAttrib() {
    unsafe { glu_sys::glPopClientAttrib() }
}

pub fn RenderMode(mode: GLenum) -> i32 {
    unsafe { glu_sys::glRenderMode(mode) }
}

pub fn GetError() -> GLenum {
    unsafe { glu_sys::glGetError() }
}

pub fn GetString(name: GLenum) -> Option<String> {
    unsafe {
        let ptr = glu_sys::glGetString(name);
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

pub fn Finish() {
    unsafe { glu_sys::glFinish() }
}

pub fn Flush() {
    unsafe { glu_sys::glFlush() }
}

pub fn Hint(target: GLenum, mode: GLenum) {
    unsafe { glu_sys::glHint(target, mode) }
}

pub fn ClearDepth(depth: f64) {
    unsafe { glu_sys::glClearDepth(depth) }
}

pub fn DepthFunc(func: GLenum) {
    unsafe { glu_sys::glDepthFunc(func) }
}

pub fn DepthMask(flag: bool) {
    unsafe { glu_sys::glDepthMask(flag as u8) }
}

pub fn DepthRange(near_val: f64, far_val: f64) {
    unsafe { glu_sys::glDepthRange(near_val, far_val) }
}

pub fn ClearAccum(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glu_sys::glClearAccum(red, green, blue, alpha) }
}

pub fn Accum(op: GLenum, value: f32) {
    unsafe { glu_sys::glAccum(op, value) }
}

pub fn MatrixMode(mode: GLenum) {
    unsafe { glu_sys::glMatrixMode(mode) }
}

pub fn Ortho(left: f64, right: f64, bottom: f64, top: f64, near_val: f64, far_val: f64) {
    unsafe { glu_sys::glOrtho(left, right, bottom, top, near_val, far_val) }
}

pub fn Frustum(left: f64, right: f64, bottom: f64, top: f64, near_val: f64, far_val: f64) {
    unsafe { glu_sys::glFrustum(left, right, bottom, top, near_val, far_val) }
}

pub fn Viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { glu_sys::glViewport(x, y, width, height) }
}

pub fn PushMatrix() {
    unsafe { glu_sys::glPushMatrix() }
}

pub fn PopMatrix() {
    unsafe { glu_sys::glPopMatrix() }
}

pub fn LoadIdentity() {
    unsafe { glu_sys::glLoadIdentity() }
}

pub fn LoadMatrixd(m: &[f64]) {
    unsafe { glu_sys::glLoadMatrixd(m.as_ptr()) }
}

pub fn LoadMatrixf(m: &[f32]) {
    unsafe { glu_sys::glLoadMatrixf(m.as_ptr()) }
}

pub fn MultMatrixd(m: &[f64]) {
    unsafe { glu_sys::glMultMatrixd(m.as_ptr()) }
}

pub fn MultMatrixf(m: &[f32]) {
    unsafe { glu_sys::glMultMatrixf(m.as_ptr()) }
}

pub fn Rotated(angle: f64, x: f64, y: f64, z: f64) {
    unsafe { glu_sys::glRotated(angle, x, y, z) }
}

pub fn Rotatef(angle: f32, x: f32, y: f32, z: f32) {
    unsafe { glu_sys::glRotatef(angle, x, y, z) }
}

pub fn Scaled(x: f64, y: f64, z: f64) {
    unsafe { glu_sys::glScaled(x, y, z) }
}

pub fn Scalef(x: f32, y: f32, z: f32) {
    unsafe { glu_sys::glScalef(x, y, z) }
}

pub fn Translated(x: f64, y: f64, z: f64) {
    unsafe { glu_sys::glTranslated(x, y, z) }
}

pub fn Translatef(x: f32, y: f32, z: f32) {
    unsafe { glu_sys::glTranslatef(x, y, z) }
}

pub fn IsList(list: u32) -> bool {
    unsafe { glu_sys::glIsList(list) != 0 }
}

pub fn DeleteLists(list: u32, range: i32) {
    unsafe { glu_sys::glDeleteLists(list, range) }
}

pub fn GenLists(range: i32) -> u32 {
    unsafe { glu_sys::glGenLists(range) }
}

pub fn NewList(list: u32, mode: GLenum) {
    unsafe { glu_sys::glNewList(list, mode) }
}

pub fn EndList() {
    unsafe { glu_sys::glEndList() }
}

pub fn CallList(list: u32) {
    unsafe { glu_sys::glCallList(list) }
}

pub unsafe fn glCallLists(n: i32, type_: GLenum, lists: *const raw::c_void) {
    unsafe { glu_sys::glCallLists(n, type_, lists) }
}

pub fn ListBase(base: u32) {
    unsafe { glu_sys::glListBase(base) }
}

pub fn Begin(mode: GLenum) {
    unsafe { glu_sys::glBegin(mode) }
}

pub fn End() {
    unsafe { glu_sys::glEnd() }
}

pub fn Vertex2d(x: f64, y: f64) {
    unsafe { glu_sys::glVertex2d(x, y) }
}

pub fn Vertex2f(x: f32, y: f32) {
    unsafe { glu_sys::glVertex2f(x, y) }
}

pub fn Vertex2i(x: i32, y: i32) {
    unsafe { glu_sys::glVertex2i(x, y) }
}

pub fn Vertex2s(x: i16, y: i16) {
    unsafe { glu_sys::glVertex2s(x, y) }
}

pub fn Vertex3d(x: f64, y: f64, z: f64) {
    unsafe { glu_sys::glVertex3d(x, y, z) }
}

pub fn Vertex3f(x: f32, y: f32, z: f32) {
    unsafe { glu_sys::glVertex3f(x, y, z) }
}

pub fn Vertex3i(x: i32, y: i32, z: i32) {
    unsafe { glu_sys::glVertex3i(x, y, z) }
}

pub fn Vertex3s(x: i16, y: i16, z: i16) {
    unsafe { glu_sys::glVertex3s(x, y, z) }
}

pub fn Vertex4d(x: f64, y: f64, z: f64, w: f64) {
    unsafe { glu_sys::glVertex4d(x, y, z, w) }
}

pub fn Vertex4f(x: f32, y: f32, z: f32, w: f32) {
    unsafe { glu_sys::glVertex4f(x, y, z, w) }
}

pub fn Vertex4i(x: i32, y: i32, z: i32, w: i32) {
    unsafe { glu_sys::glVertex4i(x, y, z, w) }
}

pub fn Vertex4s(x: i16, y: i16, z: i16, w: i16) {
    unsafe { glu_sys::glVertex4s(x, y, z, w) }
}

pub fn Vertex2dv(v: &[f64]) {
    unsafe { glu_sys::glVertex2dv(v.as_ptr()) }
}

pub fn Vertex2fv(v: &[f32]) {
    unsafe { glu_sys::glVertex2fv(v.as_ptr()) }
}

pub fn Vertex2iv(v: &[i32]) {
    unsafe { glu_sys::glVertex2iv(v.as_ptr()) }
}

pub fn Vertex2sv(v: &[i16]) {
    unsafe { glu_sys::glVertex2sv(v.as_ptr()) }
}

pub fn Vertex3dv(v: &[f64]) {
    unsafe { glu_sys::glVertex3dv(v.as_ptr()) }
}

pub fn Vertex3fv(v: &[f32]) {
    unsafe { glu_sys::glVertex3fv(v.as_ptr()) }
}

pub fn Vertex3iv(v: &[i32]) {
    unsafe { glu_sys::glVertex3iv(v.as_ptr()) }
}

pub fn Vertex3sv(v: &[i16]) {
    unsafe { glu_sys::glVertex3sv(v.as_ptr()) }
}

pub fn Vertex4dv(v: &[f64]) {
    unsafe { glu_sys::glVertex4dv(v.as_ptr()) }
}

pub fn Vertex4fv(v: &[f32]) {
    unsafe { glu_sys::glVertex4fv(v.as_ptr()) }
}

pub fn Vertex4iv(v: &[i32]) {
    unsafe { glu_sys::glVertex4iv(v.as_ptr()) }
}

pub fn Vertex4sv(v: &[i16]) {
    unsafe { glu_sys::glVertex4sv(v.as_ptr()) }
}

pub fn Normal3b(nx: i8, ny: i8, nz: i8) {
    unsafe { glu_sys::glNormal3b(nx, ny, nz) }
}

pub fn Normal3d(nx: f64, ny: f64, nz: f64) {
    unsafe { glu_sys::glNormal3d(nx, ny, nz) }
}

pub fn Normal3f(nx: f32, ny: f32, nz: f32) {
    unsafe { glu_sys::glNormal3f(nx, ny, nz) }
}

pub fn Normal3i(nx: i32, ny: i32, nz: i32) {
    unsafe { glu_sys::glNormal3i(nx, ny, nz) }
}

pub fn Normal3s(nx: i16, ny: i16, nz: i16) {
    unsafe { glu_sys::glNormal3s(nx, ny, nz) }
}

pub fn Normal3bv(v: &[i8]) {
    unsafe { glu_sys::glNormal3bv(v.as_ptr()) }
}

pub fn Normal3dv(v: &[f64]) {
    unsafe { glu_sys::glNormal3dv(v.as_ptr()) }
}

pub fn Normal3fv(v: &[f32]) {
    unsafe { glu_sys::glNormal3fv(v.as_ptr()) }
}

pub fn Normal3iv(v: &[i32]) {
    unsafe { glu_sys::glNormal3iv(v.as_ptr()) }
}

pub fn Normal3sv(v: &[i16]) {
    unsafe { glu_sys::glNormal3sv(v.as_ptr()) }
}

pub fn Indexd(c: f64) {
    unsafe { glu_sys::glIndexd(c) }
}

pub fn Indexf(c: f32) {
    unsafe { glu_sys::glIndexf(c) }
}

pub fn Indexi(c: i32) {
    unsafe { glu_sys::glIndexi(c) }
}

pub fn Indexs(c: i16) {
    unsafe { glu_sys::glIndexs(c) }
}

pub fn Indexub(c: u8) {
    unsafe { glu_sys::glIndexub(c) }
}

pub fn Indexdv(c: &[f64]) {
    unsafe { glu_sys::glIndexdv(c.as_ptr()) }
}

pub fn Indexfv(c: &[f32]) {
    unsafe { glu_sys::glIndexfv(c.as_ptr()) }
}

pub fn Indexiv(c: &[i32]) {
    unsafe { glu_sys::glIndexiv(c.as_ptr()) }
}

pub fn Indexsv(c: &[i16]) {
    unsafe { glu_sys::glIndexsv(c.as_ptr()) }
}

pub fn Indexubv(c: &[u8]) {
    unsafe { glu_sys::glIndexubv(c.as_ptr()) }
}

pub fn Color3b(red: i8, green: i8, blue: i8) {
    unsafe { glu_sys::glColor3b(red, green, blue) }
}

pub fn Color3d(red: f64, green: f64, blue: f64) {
    unsafe { glu_sys::glColor3d(red, green, blue) }
}

pub fn Color3f(red: f32, green: f32, blue: f32) {
    unsafe { glu_sys::glColor3f(red, green, blue) }
}

pub fn Color3i(red: i32, green: i32, blue: i32) {
    unsafe { glu_sys::glColor3i(red, green, blue) }
}

pub fn Color3s(red: i16, green: i16, blue: i16) {
    unsafe { glu_sys::glColor3s(red, green, blue) }
}

pub fn Color3ub(red: u8, green: u8, blue: u8) {
    unsafe { glu_sys::glColor3ub(red, green, blue) }
}

pub fn Color3ui(red: u32, green: u32, blue: u32) {
    unsafe { glu_sys::glColor3ui(red, green, blue) }
}

pub fn Color3us(red: u16, green: u16, blue: u16) {
    unsafe { glu_sys::glColor3us(red, green, blue) }
}

pub fn Color4b(red: i8, green: i8, blue: i8, alpha: i8) {
    unsafe { glu_sys::glColor4b(red, green, blue, alpha) }
}

pub fn Color4d(red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe { glu_sys::glColor4d(red, green, blue, alpha) }
}

pub fn Color4f(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glu_sys::glColor4f(red, green, blue, alpha) }
}

pub fn Color4i(red: i32, green: i32, blue: i32, alpha: i32) {
    unsafe { glu_sys::glColor4i(red, green, blue, alpha) }
}

pub fn Color4s(red: i16, green: i16, blue: i16, alpha: i16) {
    unsafe { glu_sys::glColor4s(red, green, blue, alpha) }
}

pub fn Color4ub(red: u8, green: u8, blue: u8, alpha: u8) {
    unsafe { glu_sys::glColor4ub(red, green, blue, alpha) }
}

pub fn Color4ui(red: u32, green: u32, blue: u32, alpha: u32) {
    unsafe { glu_sys::glColor4ui(red, green, blue, alpha) }
}

pub fn Color4us(red: u16, green: u16, blue: u16, alpha: u16) {
    unsafe { glu_sys::glColor4us(red, green, blue, alpha) }
}

pub fn Color3bv(v: &[i8]) {
    unsafe { glu_sys::glColor3bv(v.as_ptr()) }
}

pub fn Color3dv(v: &[f64]) {
    unsafe { glu_sys::glColor3dv(v.as_ptr()) }
}

pub fn Color3fv(v: &[f32]) {
    unsafe { glu_sys::glColor3fv(v.as_ptr()) }
}

pub fn Color3iv(v: &[i32]) {
    unsafe { glu_sys::glColor3iv(v.as_ptr()) }
}

pub fn Color3sv(v: &[i16]) {
    unsafe { glu_sys::glColor3sv(v.as_ptr()) }
}

pub fn Color3ubv(v: &[u8]) {
    unsafe { glu_sys::glColor3ubv(v.as_ptr()) }
}

pub fn Color3uiv(v: &[u32]) {
    unsafe { glu_sys::glColor3uiv(v.as_ptr()) }
}

pub fn Color3usv(v: &[u16]) {
    unsafe { glu_sys::glColor3usv(v.as_ptr()) }
}

pub fn Color4bv(v: &[i8]) {
    unsafe { glu_sys::glColor4bv(v.as_ptr()) }
}

pub fn Color4dv(v: &[f64]) {
    unsafe { glu_sys::glColor4dv(v.as_ptr()) }
}

pub fn Color4fv(v: &[f32]) {
    unsafe { glu_sys::glColor4fv(v.as_ptr()) }
}

pub fn Color4iv(v: &[i32]) {
    unsafe { glu_sys::glColor4iv(v.as_ptr()) }
}

pub fn Color4sv(v: &[i16]) {
    unsafe { glu_sys::glColor4sv(v.as_ptr()) }
}

pub fn Color4ubv(v: &[u8]) {
    unsafe { glu_sys::glColor4ubv(v.as_ptr()) }
}

pub fn Color4uiv(v: &[u32]) {
    unsafe { glu_sys::glColor4uiv(v.as_ptr()) }
}

pub fn Color4usv(v: &[u16]) {
    unsafe { glu_sys::glColor4usv(v.as_ptr()) }
}

pub fn TexCoord1d(s: f64) {
    unsafe { glu_sys::glTexCoord1d(s) }
}

pub fn TexCoord1f(s: f32) {
    unsafe { glu_sys::glTexCoord1f(s) }
}

pub fn TexCoord1i(s: i32) {
    unsafe { glu_sys::glTexCoord1i(s) }
}

pub fn TexCoord1s(s: i16) {
    unsafe { glu_sys::glTexCoord1s(s) }
}

pub fn TexCoord2d(s: f64, t: f64) {
    unsafe { glu_sys::glTexCoord2d(s, t) }
}

pub fn TexCoord2f(s: f32, t: f32) {
    unsafe { glu_sys::glTexCoord2f(s, t) }
}

pub fn TexCoord2i(s: i32, t: i32) {
    unsafe { glu_sys::glTexCoord2i(s, t) }
}

pub fn TexCoord2s(s: i16, t: i16) {
    unsafe { glu_sys::glTexCoord2s(s, t) }
}

pub fn TexCoord3d(s: f64, t: f64, r: f64) {
    unsafe { glu_sys::glTexCoord3d(s, t, r) }
}

pub fn TexCoord3f(s: f32, t: f32, r: f32) {
    unsafe { glu_sys::glTexCoord3f(s, t, r) }
}

pub fn TexCoord3i(s: i32, t: i32, r: i32) {
    unsafe { glu_sys::glTexCoord3i(s, t, r) }
}

pub fn TexCoord3s(s: i16, t: i16, r: i16) {
    unsafe { glu_sys::glTexCoord3s(s, t, r) }
}

pub fn TexCoord4d(s: f64, t: f64, r: f64, q: f64) {
    unsafe { glu_sys::glTexCoord4d(s, t, r, q) }
}

pub fn TexCoord4f(s: f32, t: f32, r: f32, q: f32) {
    unsafe { glu_sys::glTexCoord4f(s, t, r, q) }
}

pub fn TexCoord4i(s: i32, t: i32, r: i32, q: i32) {
    unsafe { glu_sys::glTexCoord4i(s, t, r, q) }
}

pub fn TexCoord4s(s: i16, t: i16, r: i16, q: i16) {
    unsafe { glu_sys::glTexCoord4s(s, t, r, q) }
}

pub fn TexCoord1dv(v: &[f64]) {
    unsafe { glu_sys::glTexCoord1dv(v.as_ptr()) }
}

pub fn TexCoord1fv(v: &[f32]) {
    unsafe { glu_sys::glTexCoord1fv(v.as_ptr()) }
}

pub fn TexCoord1iv(v: &[i32]) {
    unsafe { glu_sys::glTexCoord1iv(v.as_ptr()) }
}

pub fn TexCoord1sv(v: &[i16]) {
    unsafe { glu_sys::glTexCoord1sv(v.as_ptr()) }
}

pub fn TexCoord2dv(v: &[f64]) {
    unsafe { glu_sys::glTexCoord2dv(v.as_ptr()) }
}

pub fn TexCoord2fv(v: &[f32]) {
    unsafe { glu_sys::glTexCoord2fv(v.as_ptr()) }
}

pub fn TexCoord2iv(v: &[i32]) {
    unsafe { glu_sys::glTexCoord2iv(v.as_ptr()) }
}

pub fn TexCoord2sv(v: &[i16]) {
    unsafe { glu_sys::glTexCoord2sv(v.as_ptr()) }
}

pub fn TexCoord3dv(v: &[f64]) {
    unsafe { glu_sys::glTexCoord3dv(v.as_ptr()) }
}

pub fn TexCoord3fv(v: &[f32]) {
    unsafe { glu_sys::glTexCoord3fv(v.as_ptr()) }
}

pub fn TexCoord3iv(v: &[i32]) {
    unsafe { glu_sys::glTexCoord3iv(v.as_ptr()) }
}

pub fn TexCoord3sv(v: &[i16]) {
    unsafe { glu_sys::glTexCoord3sv(v.as_ptr()) }
}

pub fn TexCoord4dv(v: &[f64]) {
    unsafe { glu_sys::glTexCoord4dv(v.as_ptr()) }
}

pub fn TexCoord4fv(v: &[f32]) {
    unsafe { glu_sys::glTexCoord4fv(v.as_ptr()) }
}

pub fn TexCoord4iv(v: &[i32]) {
    unsafe { glu_sys::glTexCoord4iv(v.as_ptr()) }
}

pub fn TexCoord4sv(v: &[i16]) {
    unsafe { glu_sys::glTexCoord4sv(v.as_ptr()) }
}

pub fn RasterPos2d(x: f64, y: f64) {
    unsafe { glu_sys::glRasterPos2d(x, y) }
}

pub fn RasterPos2f(x: f32, y: f32) {
    unsafe { glu_sys::glRasterPos2f(x, y) }
}

pub fn RasterPos2i(x: i32, y: i32) {
    unsafe { glu_sys::glRasterPos2i(x, y) }
}

pub fn RasterPos2s(x: i16, y: i16) {
    unsafe { glu_sys::glRasterPos2s(x, y) }
}

pub fn RasterPos3d(x: f64, y: f64, z: f64) {
    unsafe { glu_sys::glRasterPos3d(x, y, z) }
}

pub fn RasterPos3f(x: f32, y: f32, z: f32) {
    unsafe { glu_sys::glRasterPos3f(x, y, z) }
}

pub fn RasterPos3i(x: i32, y: i32, z: i32) {
    unsafe { glu_sys::glRasterPos3i(x, y, z) }
}

pub fn RasterPos3s(x: i16, y: i16, z: i16) {
    unsafe { glu_sys::glRasterPos3s(x, y, z) }
}

pub fn RasterPos4d(x: f64, y: f64, z: f64, w: f64) {
    unsafe { glu_sys::glRasterPos4d(x, y, z, w) }
}

pub fn RasterPos4f(x: f32, y: f32, z: f32, w: f32) {
    unsafe { glu_sys::glRasterPos4f(x, y, z, w) }
}

pub fn RasterPos4i(x: i32, y: i32, z: i32, w: i32) {
    unsafe { glu_sys::glRasterPos4i(x, y, z, w) }
}

pub fn RasterPos4s(x: i16, y: i16, z: i16, w: i16) {
    unsafe { glu_sys::glRasterPos4s(x, y, z, w) }
}

pub fn RasterPos2dv(v: &[f64]) {
    unsafe { glu_sys::glRasterPos2dv(v.as_ptr()) }
}

pub fn RasterPos2fv(v: &[f32]) {
    unsafe { glu_sys::glRasterPos2fv(v.as_ptr()) }
}

pub fn RasterPos2iv(v: &[i32]) {
    unsafe { glu_sys::glRasterPos2iv(v.as_ptr()) }
}

pub fn RasterPos2sv(v: &[i16]) {
    unsafe { glu_sys::glRasterPos2sv(v.as_ptr()) }
}

pub fn RasterPos3dv(v: &[f64]) {
    unsafe { glu_sys::glRasterPos3dv(v.as_ptr()) }
}

pub fn RasterPos3fv(v: &[f32]) {
    unsafe { glu_sys::glRasterPos3fv(v.as_ptr()) }
}

pub fn RasterPos3iv(v: &[i32]) {
    unsafe { glu_sys::glRasterPos3iv(v.as_ptr()) }
}

pub fn RasterPos3sv(v: &[i16]) {
    unsafe { glu_sys::glRasterPos3sv(v.as_ptr()) }
}

pub fn RasterPos4dv(v: &[f64]) {
    unsafe { glu_sys::glRasterPos4dv(v.as_ptr()) }
}

pub fn RasterPos4fv(v: &[f32]) {
    unsafe { glu_sys::glRasterPos4fv(v.as_ptr()) }
}

pub fn RasterPos4iv(v: &[i32]) {
    unsafe { glu_sys::glRasterPos4iv(v.as_ptr()) }
}

pub fn RasterPos4sv(v: &[i16]) {
    unsafe { glu_sys::glRasterPos4sv(v.as_ptr()) }
}

pub fn Rectd(x1: f64, y1: f64, x2: f64, y2: f64) {
    unsafe { glu_sys::glRectd(x1, y1, x2, y2) }
}

pub fn Rectf(x1: f32, y1: f32, x2: f32, y2: f32) {
    unsafe { glu_sys::glRectf(x1, y1, x2, y2) }
}

pub fn Recti(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { glu_sys::glRecti(x1, y1, x2, y2) }
}

pub fn Rects(x1: i16, y1: i16, x2: i16, y2: i16) {
    unsafe { glu_sys::glRects(x1, y1, x2, y2) }
}

pub fn Rectdv(v1: &[f64], v2: &[f64]) {
    unsafe { glu_sys::glRectdv(v1.as_ptr(), v2.as_ptr()) }
}

pub fn Rectfv(v1: &[f32], v2: &[f32]) {
    unsafe { glu_sys::glRectfv(v1.as_ptr(), v2.as_ptr()) }
}

pub fn Rectiv(v1: &[i32], v2: &[i32]) {
    unsafe { glu_sys::glRectiv(v1.as_ptr(), v2.as_ptr()) }
}

pub fn Rectsv(v1: &[i16], v2: &[i16]) {
    unsafe { glu_sys::glRectsv(v1.as_ptr(), v2.as_ptr()) }
}

pub fn VertexPointer(size: i32, type_: GLenum, stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glVertexPointer(size, type_, stride, ptr) }
}

pub fn NormalPointer(type_: GLenum, stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glNormalPointer(type_, stride, ptr) }
}

pub fn ColorPointer(size: i32, type_: GLenum, stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glColorPointer(size, type_, stride, ptr) }
}

pub fn IndexPointer(type_: GLenum, stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glIndexPointer(type_, stride, ptr) }
}

pub fn TexCoordPointer(size: i32, type_: GLenum, stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glTexCoordPointer(size, type_, stride, ptr) }
}

pub fn EdgeFlagPointer(stride: i32, ptr: *const raw::c_void) {
    unsafe { glu_sys::glEdgeFlagPointer(stride, ptr) }
}

pub fn GetPointerv(pname: GLenum, params: *mut *mut raw::c_void) {
    unsafe { glu_sys::glGetPointerv(pname, params) }
}

pub fn ArrayElement(i: i32) {
    unsafe { glu_sys::glArrayElement(i) }
}

pub fn DrawArrays(mode: GLenum, first: i32, count: i32) {
    unsafe { glu_sys::glDrawArrays(mode, first, count) }
}

pub fn DrawElements(mode: GLenum, count: i32, type_: GLenum, indices: *const raw::c_void) {
    unsafe { glu_sys::glDrawElements(mode, count, type_, indices) }
}

pub fn i32erleavedArrays(format: u32, stride: i32, pointer: *const raw::c_void) {
    unsafe { glu_sys::glInterleavedArrays(format, stride, pointer) }
}

pub fn ShadeModel(mode: GLenum) {
    unsafe { glu_sys::glShadeModel(mode) }
}

pub fn Lightf(light: GLenum, pname: GLenum, param: f32) {
    unsafe { glu_sys::glLightf(light, pname, param) }
}

pub fn Lighti(light: GLenum, pname: GLenum, param: i32) {
    unsafe { glu_sys::glLighti(light, pname, param) }
}

pub fn Lightfv(light: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glLightfv(light, pname, params.as_ptr()) }
}

pub fn Lightiv(light: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glLightiv(light, pname, params.as_ptr()) }
}

pub fn GetLightfv(light: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetLightfv(light, pname, params.as_mut_ptr());
    }
}

pub fn GetLightiv(light: GLenum, pname: GLenum, params: &mut [i32])  {
    unsafe {
        glu_sys::glGetLightiv(light, pname, params.as_mut_ptr());
    }
}

pub fn LightModelf(pname: GLenum, param: f32) {
    unsafe { glu_sys::glLightModelf(pname, param) }
}

pub fn LightModeli(pname: GLenum, param: i32) {
    unsafe { glu_sys::glLightModeli(pname, param) }
}

pub fn LightModelfv(pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glLightModelfv(pname, params.as_ptr()) }
}

pub fn LightModeliv(pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glLightModeliv(pname, params.as_ptr()) }
}

pub fn Materialf(face: GLenum, pname: GLenum, param: f32) {
    unsafe { glu_sys::glMaterialf(face, pname, param) }
}

pub fn Materiali(face: GLenum, pname: GLenum, param: i32) {
    unsafe { glu_sys::glMateriali(face, pname, param) }
}

pub fn Materialfv(face: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glMaterialfv(face, pname, params.as_ptr()) }
}

pub fn Materialiv(face: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glMaterialiv(face, pname, params.as_ptr()) }
}

pub fn GetMaterialfv(face: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetMaterialfv(face, pname, params.as_mut_ptr());
    }
}

pub fn GetMaterialiv(face: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetMaterialiv(face, pname, params.as_mut_ptr());
    }
}

pub fn ColorMaterial(face: GLenum, mode: GLenum) {
    unsafe { glu_sys::glColorMaterial(face, mode) }
}

pub fn PixelZoom(xfactor: f32, yfactor: f32) {
    unsafe { glu_sys::glPixelZoom(xfactor, yfactor) }
}

pub fn PixelStoref(pname: GLenum, param: f32) {
    unsafe { glu_sys::glPixelStoref(pname, param) }
}

pub fn PixelStorei(pname: GLenum, param: i32) {
    unsafe { glu_sys::glPixelStorei(pname, param) }
}

pub fn PixelTransferf(pname: GLenum, param: f32) {
    unsafe { glu_sys::glPixelTransferf(pname, param) }
}

pub fn PixelTransferi(pname: GLenum, param: i32) {
    unsafe { glu_sys::glPixelTransferi(pname, param) }
}

pub fn PixelMapfv(map: GLenum, mapsize: i32, values: &[f32]) {
    unsafe { glu_sys::glPixelMapfv(map, mapsize, values.as_ptr()) }
}

pub fn PixelMapuiv(map: GLenum, mapsize: i32, values: &[u32]) {
    unsafe { glu_sys::glPixelMapuiv(map, mapsize, values.as_ptr()) }
}

pub fn PixelMapusv(map: GLenum, mapsize: i32, values: &[u16]) {
    unsafe { glu_sys::glPixelMapusv(map, mapsize, values.as_ptr()) }
}

pub fn GetPixelMapfv(map: GLenum, values: &mut [f32]) {
    unsafe {
        glu_sys::glGetPixelMapfv(map, values.as_mut_ptr());
    }
}

pub fn GetPixelMapuiv(map: GLenum, values: &mut [u32]) {
    unsafe {
        glu_sys::glGetPixelMapuiv(map, values.as_mut_ptr());
    }
}

pub fn GetPixelMapusv(map: GLenum, values: &mut [u16])  {
    unsafe {
        glu_sys::glGetPixelMapusv(map, values.as_mut_ptr());
    }
}

pub fn Bitmap(
    width: i32,
    height: i32,
    xorig: f32,
    yorig: f32,
    xmove: f32,
    ymove: f32,
    bitmap: &[u8],
) {
    unsafe { glu_sys::glBitmap(width, height, xorig, yorig, xmove, ymove, bitmap.as_ptr()) }
}

pub fn ReadPixels(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    type_: GLenum,
    pixels: *mut raw::c_void,
) {
    unsafe { glu_sys::glReadPixels(x, y, width, height, format, type_, pixels) }
}

pub fn DrawPixels(
    width: i32,
    height: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe { glu_sys::glDrawPixels(width, height, format, type_, pixels) }
}

pub fn CopyPixels(x: i32, y: i32, width: i32, height: i32, type_: GLenum) {
    unsafe { glu_sys::glCopyPixels(x, y, width, height, type_) }
}

pub fn StencilFunc(func: u32, ref_: i32, mask: u32) {
    unsafe { glu_sys::glStencilFunc(func, ref_, mask) }
}

pub fn StencilMask(mask: u32) {
    unsafe { glu_sys::glStencilMask(mask) }
}

pub fn StencilOp(fail: u32, zfail: u32, zpass: u32) {
    unsafe { glu_sys::glStencilOp(fail, zfail, zpass) }
}

pub fn ClearStencil(s: i32) {
    unsafe { glu_sys::glClearStencil(s) }
}

pub fn TexGend(coord: GLenum, pname: GLenum, param: f64) {
    unsafe { glu_sys::glTexGend(coord, pname, param) }
}

pub fn TexGenf(coord: GLenum, pname: GLenum, param: f32) {
    unsafe { glu_sys::glTexGenf(coord, pname, param) }
}

pub fn TexGeni(coord: GLenum, pname: GLenum, param: i32) {
    unsafe { glu_sys::glTexGeni(coord, pname, param) }
}

pub fn TexGendv(coord: GLenum, pname: GLenum, params: &[f64]) {
    unsafe { glu_sys::glTexGendv(coord, pname, params.as_ptr()) }
}

pub fn TexGenfv(coord: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glTexGenfv(coord, pname, params.as_ptr()) }
}

pub fn TexGeniv(coord: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glTexGeniv(coord, pname, params.as_ptr()) }
}

pub fn GetTexGendv(coord: GLenum, pname: GLenum, params: &mut [f64]) {
    unsafe {
        glu_sys::glGetTexGendv(coord, pname, params.as_mut_ptr());
    }
}

pub fn GetTexGenfv(coord: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetTexGenfv(coord, pname, params.as_mut_ptr());
    }
}

pub fn GetTexGeniv(coord: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetTexGeniv(coord, pname, params.as_mut_ptr());
    }
}

pub fn TexEnvf(target: GLenum, pname: GLenum, param: f32) {
    unsafe { glu_sys::glTexEnvf(target, pname, param) }
}

pub fn TexEnvi(target: GLenum, pname: GLenum, param: i32) {
    unsafe { glu_sys::glTexEnvi(target, pname, param) }
}

pub fn TexEnvfv(target: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glTexEnvfv(target, pname, params.as_ptr()) }
}

pub fn TexEnviv(target: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glTexEnviv(target, pname, params.as_ptr()) }
}

pub fn GetTexEnvfv(target: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetTexEnvfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetTexEnviv(target: GLenum, pname: GLenum, params: &mut [i32]){
    unsafe {
        glu_sys::glGetTexEnviv(target, pname, params.as_mut_ptr());
    }
}

pub fn TexParameterf(target: GLenum, pname: GLenum, param: f32) {
    unsafe { glu_sys::glTexParameterf(target, pname, param) }
}

pub fn TexParameteri(target: GLenum, pname: GLenum, param: i32) {
    unsafe { glu_sys::glTexParameteri(target, pname, param) }
}

pub fn TexParameterfv(target: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glTexParameterfv(target, pname, params.as_ptr()) }
}

pub fn TexParameteriv(target: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glTexParameteriv(target, pname, params.as_ptr()) }
}

pub fn GetTexParameterfv(target: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetTexParameterfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetTexParameteriv(target: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetTexParameteriv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetTexLevelParameterfv(target: GLenum, level: i32, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetTexLevelParameterfv(target, level, pname, params.as_mut_ptr());
    }
}

pub fn GetTexLevelParameteriv(target: GLenum, level: i32, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetTexLevelParameteriv(target, level, pname, params.as_mut_ptr());
    }
}

pub fn TexImage1D(
    target: GLenum,
    level: i32,
    internalFormat: i32,
    width: i32,
    border: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe {
        glu_sys::glTexImage1D(
            target,
            level,
            internalFormat,
            width,
            border,
            format,
            type_,
            pixels,
        )
    }
}

pub fn TexImage2D(
    target: GLenum,
    level: i32,
    internalFormat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe {
        glu_sys::glTexImage2D(
            target,
            level,
            internalFormat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        )
    }
}

pub fn GetTexImage(
    target: GLenum,
    level: i32,
    format: u32,
    type_: GLenum,
    pixels: *mut raw::c_void,
) {
    unsafe { glu_sys::glGetTexImage(target, level, format, type_, pixels) }
}

pub fn GenTextures(n: i32, textures: &mut [u32]) {
    unsafe {
        glu_sys::glGenTextures(n, textures.as_mut_ptr());
    }
}

pub fn DeleteTextures(n: i32, textures: &[u32]) {
    unsafe { glu_sys::glDeleteTextures(n, textures.as_ptr()) }
}

pub fn BindTexture(target: GLenum, texture: GLenum) {
    unsafe { glu_sys::glBindTexture(target, texture) }
}

pub fn PrioritizeTextures(n: i32, textures: &[u32], priorities: &[f32]) {
    unsafe { glu_sys::glPrioritizeTextures(n, textures.as_ptr(), priorities.as_ptr()) }
}

pub fn AreTexturesResident(n: i32, textures: &[u32], residences: &mut [bool]) -> bool {
    unsafe {
        glu_sys::glAreTexturesResident(n, textures.as_ptr(), residences.as_mut_ptr() as *mut u8) != 0
    }
}

pub fn IsTexture(texture: GLenum) -> bool {
    unsafe { glu_sys::glIsTexture(texture) != 0 }
}

pub fn TexSubImage1D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    width: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe { glu_sys::glTexSubImage1D(target, level, xoffset, width, format, type_, pixels) }
}

pub fn TexSubImage2D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe {
        glu_sys::glTexSubImage2D(
            target, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }
}

pub fn CopyTexImage1D(
    target: GLenum,
    level: i32,
    internalformat: GLenum,
    x: i32,
    y: i32,
    width: i32,
    border: i32,
) {
    unsafe { glu_sys::glCopyTexImage1D(target, level, internalformat, x, y, width, border) }
}

pub fn CopyTexImage2D(
    target: GLenum,
    level: i32,
    internalformat: GLenum,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
) {
    unsafe { glu_sys::glCopyTexImage2D(target, level, internalformat, x, y, width, height, border) }
}

pub fn CopyTexSubImage1D(target: GLenum, level: i32, xoffset: i32, x: i32, y: i32, width: i32) {
    unsafe { glu_sys::glCopyTexSubImage1D(target, level, xoffset, x, y, width) }
}

pub fn CopyTexSubImage2D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe { glu_sys::glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
}

pub fn Map1d(target: GLenum, u1: f64, u2: f64, stride: i32, order: i32, points: &[f64]) {
    unsafe { glu_sys::glMap1d(target, u1, u2, stride, order, points.as_ptr()) }
}

pub fn Map1f(target: GLenum, u1: f32, u2: f32, stride: i32, order: i32, points: &[f32]) {
    unsafe { glu_sys::glMap1f(target, u1, u2, stride, order, points.as_ptr()) }
}

pub fn Map2d(
    target: GLenum,
    u1: f64,
    u2: f64,
    ustride: i32,
    uorder: i32,
    v1: f64,
    v2: f64,
    vstride: i32,
    vorder: i32,
    points: &[f64],
) {
    unsafe {
        glu_sys::glMap2d(
            target,
            u1,
            u2,
            ustride,
            uorder,
            v1,
            v2,
            vstride,
            vorder,
            points.as_ptr(),
        )
    }
}

pub fn Map2f(
    target: GLenum,
    u1: f32,
    u2: f32,
    ustride: i32,
    uorder: i32,
    v1: f32,
    v2: f32,
    vstride: i32,
    vorder: i32,
    points: &[f32],
) {
    unsafe {
        glu_sys::glMap2f(
            target,
            u1,
            u2,
            ustride,
            uorder,
            v1,
            v2,
            vstride,
            vorder,
            points.as_ptr(),
        )
    }
}

pub fn GetMapdv(target: GLenum, query: GLenum, v: &mut [f64]) {
    unsafe {
        glu_sys::glGetMapdv(target, query, v.as_mut_ptr());
    }
}

pub fn GetMapfv(target: GLenum, query: GLenum, v: &mut [f32]) {
    unsafe {
        glu_sys::glGetMapfv(target, query, v.as_mut_ptr());
    }
}

pub fn GetMapiv(target: GLenum, query: GLenum, v: &mut [i32]) {
    unsafe {
        glu_sys::glGetMapiv(target, query, v.as_mut_ptr());
    }
}

pub fn EvalCoord1d(u: f64) {
    unsafe { glu_sys::glEvalCoord1d(u) }
}

pub fn EvalCoord1f(u: f32) {
    unsafe { glu_sys::glEvalCoord1f(u) }
}

pub fn EvalCoord1dv(u: &[f64]) {
    unsafe { glu_sys::glEvalCoord1dv(u.as_ptr()) }
}

pub fn EvalCoord1fv(u: &[f32]) {
    unsafe { glu_sys::glEvalCoord1fv(u.as_ptr()) }
}

pub fn EvalCoord2d(u: f64, v: f64) {
    unsafe { glu_sys::glEvalCoord2d(u, v) }
}

pub fn EvalCoord2f(u: f32, v: f32) {
    unsafe { glu_sys::glEvalCoord2f(u, v) }
}

pub fn EvalCoord2dv(u: &[f64]) {
    unsafe { glu_sys::glEvalCoord2dv(u.as_ptr()) }
}

pub fn EvalCoord2fv(u: &[f32]) {
    unsafe { glu_sys::glEvalCoord2fv(u.as_ptr()) }
}

pub fn MapGrid1d(un: i32, u1: f64, u2: f64) {
    unsafe { glu_sys::glMapGrid1d(un, u1, u2) }
}

pub fn MapGrid1f(un: i32, u1: f32, u2: f32) {
    unsafe { glu_sys::glMapGrid1f(un, u1, u2) }
}

pub fn MapGrid2d(un: i32, u1: f64, u2: f64, vn: i32, v1: f64, v2: f64) {
    unsafe { glu_sys::glMapGrid2d(un, u1, u2, vn, v1, v2) }
}

pub fn MapGrid2f(un: i32, u1: f32, u2: f32, vn: i32, v1: f32, v2: f32) {
    unsafe { glu_sys::glMapGrid2f(un, u1, u2, vn, v1, v2) }
}

pub fn EvalPoint1(i: i32) {
    unsafe { glu_sys::glEvalPoint1(i) }
}

pub fn EvalPoint2(i: i32, j: i32) {
    unsafe { glu_sys::glEvalPoint2(i, j) }
}

pub fn EvalMesh1(mode: GLenum, i1: i32, i2: i32) {
    unsafe { glu_sys::glEvalMesh1(mode, i1, i2) }
}

pub fn EvalMesh2(mode: GLenum, i1: i32, i2: i32, j1: i32, j2: i32) {
    unsafe { glu_sys::glEvalMesh2(mode, i1, i2, j1, j2) }
}

pub fn Fogf(pname: GLenum, param: f32) {
    unsafe { glu_sys::glFogf(pname, param) }
}

pub fn Fogi(pname: GLenum, param: i32) {
    unsafe { glu_sys::glFogi(pname, param) }
}

pub fn Fogfv(pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glFogfv(pname, params.as_ptr()) }
}

pub fn Fogiv(pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glFogiv(pname, params.as_ptr()) }
}

pub fn FeedbackBuffer(size: i32, type_: GLenum, buffer: &mut [f32]) {
    unsafe {
        glu_sys::glFeedbackBuffer(size, type_, buffer.as_mut_ptr());
    }
}

pub fn PassThrough(token: f32) {
    unsafe { glu_sys::glPassThrough(token) }
}

pub fn SelectBuffer(size: i32, buffer: &mut [u32]) {
    unsafe {
        glu_sys::glSelectBuffer(size, buffer.as_mut_ptr());
    }
}

pub fn InitNames() {
    unsafe { glu_sys::glInitNames() }
}

pub fn LoadName(name: u32) {
    unsafe { glu_sys::glLoadName(name) }
}

pub fn PushName(name: u32) {
    unsafe { glu_sys::glPushName(name) }
}

pub fn PopName() {
    unsafe { glu_sys::glPopName() }
}

pub fn DrawRangeElements(
    mode: GLenum,
    start: u32,
    end: u32,
    count: i32,
    type_: GLenum,
    indices: *const raw::c_void,
) {
    unsafe { glu_sys::glDrawRangeElements(mode, start, end, count, type_, indices) }
}

pub fn TexImage3D(
    target: GLenum,
    level: i32,
    internalFormat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe {
        glu_sys::glTexImage3D(
            target,
            level,
            internalFormat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels,
        )
    }
}

pub fn TexSubImage3D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: GLenum,
    pixels: *const raw::c_void,
) {
    unsafe {
        glu_sys::glTexSubImage3D(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }
}

pub fn CopyTexSubImage3D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        glu_sys::glCopyTexSubImage3D(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
}

pub fn ColorTable(
    target: GLenum,
    internalformat: GLenum,
    width: i32,
    format: u32,
    type_: GLenum,
    table: *const raw::c_void,
) {
    unsafe { glu_sys::glColorTable(target, internalformat, width, format, type_, table) }
}

pub fn ColorSubTable(
    target: GLenum,
    start: i32,
    count: i32,
    format: u32,
    type_: GLenum,
    data: *const raw::c_void,
) {
    unsafe { glu_sys::glColorSubTable(target, start, count, format, type_, data) }
}

pub fn ColorTableParameteriv(target: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glColorTableParameteriv(target, pname, params.as_ptr()) }
}

pub fn ColorTableParameterfv(target: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glColorTableParameterfv(target, pname, params.as_ptr()) }
}

pub fn CopyColorSubTable(target: GLenum, start: i32, x: i32, y: i32, width: i32) {
    unsafe { glu_sys::glCopyColorSubTable(target, start, x, y, width) }
}

pub fn CopyColorTable(target: GLenum, internalformat: GLenum, x: i32, y: i32, width: i32) {
    unsafe { glu_sys::glCopyColorTable(target, internalformat, x, y, width) }
}

pub fn GetColorTable(target: GLenum, format: u32, type_: GLenum, table: *mut raw::c_void) {
    unsafe { glu_sys::glGetColorTable(target, format, type_, table) }
}

pub fn GetColorTableParameterfv(target: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetColorTableParameterfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetColorTableParameteriv(target: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetColorTableParameteriv(target, pname, params.as_mut_ptr());
    }
}

pub fn BlendEquation(mode: GLenum) {
    unsafe { glu_sys::glBlendEquation(mode) }
}

pub fn BlendColor(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glu_sys::glBlendColor(red, green, blue, alpha) }
}

pub fn Histogram(target: GLenum, width: i32, internalformat: GLenum, sink: bool) {
    unsafe { glu_sys::glHistogram(target, width, internalformat, sink as u8) }
}

pub fn ResetHistogram(target: GLenum) {
    unsafe { glu_sys::glResetHistogram(target) }
}

pub fn GetHistogram(
    target: GLenum,
    reset: bool,
    format: u32,
    type_: GLenum,
    values: *mut raw::c_void,
) {
    unsafe { glu_sys::glGetHistogram(target, reset as u8, format, type_, values) }
}

pub fn GetHistogramParameterfv(target: GLenum, pname: GLenum, params: &mut [f32]){
    unsafe {
        glu_sys::glGetHistogramParameterfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetHistogramParameteriv(target: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetHistogramParameteriv(target, pname, params.as_mut_ptr());
    }
}

pub fn Minmax(target: GLenum, internalformat: GLenum, sink: bool) {
    unsafe { glu_sys::glMinmax(target, internalformat, sink as u8) }
}

pub fn ResetMinmax(target: GLenum) {
    unsafe { glu_sys::glResetMinmax(target) }
}

pub fn GetMinmax(target: GLenum, reset: bool, format: u32, types: u32, values: *mut raw::c_void) {
    unsafe { glu_sys::glGetMinmax(target, reset as u8, format, types, values) }
}

pub fn GetMinmaxParameterfv(target: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetMinmaxParameterfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetMinmaxParameteriv(target: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetMinmaxParameteriv(target, pname, params.as_mut_ptr());
    }
}

pub fn ConvolutionFilter1D(
    target: GLenum,
    internalformat: GLenum,
    width: i32,
    format: u32,
    type_: GLenum,
    image: *const raw::c_void,
) {
    unsafe { glu_sys::glConvolutionFilter1D(target, internalformat, width, format, type_, image) }
}

pub fn ConvolutionFilter2D(
    target: GLenum,
    internalformat: GLenum,
    width: i32,
    height: i32,
    format: u32,
    type_: GLenum,
    image: *const raw::c_void,
) {
    unsafe {
        glu_sys::glConvolutionFilter2D(target, internalformat, width, height, format, type_, image)
    }
}

pub fn ConvolutionParameterf(target: GLenum, pname: GLenum, params: f32) {
    unsafe { glu_sys::glConvolutionParameterf(target, pname, params) }
}

pub fn ConvolutionParameterfv(target: GLenum, pname: GLenum, params: &[f32]) {
    unsafe { glu_sys::glConvolutionParameterfv(target, pname, params.as_ptr()) }
}

pub fn ConvolutionParameteri(target: GLenum, pname: GLenum, params: i32) {
    unsafe { glu_sys::glConvolutionParameteri(target, pname, params) }
}

pub fn ConvolutionParameteriv(target: GLenum, pname: GLenum, params: &[i32]) {
    unsafe { glu_sys::glConvolutionParameteriv(target, pname, params.as_ptr()) }
}

pub fn CopyConvolutionFilter1D(
    target: GLenum,
    internalformat: GLenum,
    x: i32,
    y: i32,
    width: i32,
) {
    unsafe { glu_sys::glCopyConvolutionFilter1D(target, internalformat, x, y, width) }
}

pub fn CopyConvolutionFilter2D(
    target: GLenum,
    internalformat: GLenum,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe { glu_sys::glCopyConvolutionFilter2D(target, internalformat, x, y, width, height) }
}

pub fn GetConvolutionFilter(target: GLenum, format: u32, type_: GLenum, image: *mut raw::c_void) {
    unsafe { glu_sys::glGetConvolutionFilter(target, format, type_, image) }
}

pub fn GetConvolutionParameterfv(target: GLenum, pname: GLenum, params: &mut [f32]) {
    unsafe {
        glu_sys::glGetConvolutionParameterfv(target, pname, params.as_mut_ptr());
    }
}

pub fn GetConvolutionParameteriv(target: GLenum, pname: GLenum, params: &mut [i32]) {
    unsafe {
        glu_sys::glGetConvolutionParameteriv(target, pname, params.as_mut_ptr());
    }
}

pub fn SeparableFilter2D(
    target: GLenum,
    internalformat: GLenum,
    width: i32,
    height: i32,
    format: u32,
    type_: GLenum,
    row: *const raw::c_void,
    column: *const raw::c_void,
) {
    unsafe {
        glu_sys::glSeparableFilter2D(
            target,
            internalformat,
            width,
            height,
            format,
            type_,
            row,
            column,
        )
    }
}

pub fn GetSeparableFilter(
    target: GLenum,
    format: u32,
    type_: GLenum,
    row: *mut raw::c_void,
    column: *mut raw::c_void,
    span: *mut raw::c_void,
) {
    unsafe { glu_sys::glGetSeparableFilter(target, format, type_, row, column, span) }
}

pub fn ActiveTexture(texture: GLenum) {
    unsafe { glu_sys::glActiveTexture(texture) }
}

pub fn ClientActiveTexture(texture: GLenum) {
    unsafe { glu_sys::glClientActiveTexture(texture) }
}

pub fn CompressedTexImage1D(
    target: GLenum,
    level: i32,
    internalformat: GLenum,
    width: i32,
    border: i32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexImage1D(
            target,
            level,
            internalformat,
            width,
            border,
            imageSize,
            data,
        )
    }
}

pub fn CompressedTexImage2D(
    target: GLenum,
    level: i32,
    internalformat: GLenum,
    width: i32,
    height: i32,
    border: i32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
}

pub fn CompressedTexImage3D(
    target: GLenum,
    level: i32,
    internalformat: GLenum,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexImage3D(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    }
}

pub fn CompressedTexSubImage1D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    width: i32,
    format: u32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexSubImage1D(target, level, xoffset, width, format, imageSize, data)
    }
}

pub fn CompressedTexSubImage2D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexSubImage2D(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
}

pub fn CompressedTexSubImage3D(
    target: GLenum,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    imageSize: i32,
    data: *const raw::c_void,
) {
    unsafe {
        glu_sys::glCompressedTexSubImage3D(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    }
}

pub fn GetCompressedTexImage(target: GLenum, lod: i32, img: *mut raw::c_void) {
    unsafe { glu_sys::glGetCompressedTexImage(target, lod, img) }
}

pub fn MultiTexCoord1d(target: GLenum, s: f64) {
    unsafe { glu_sys::glMultiTexCoord1d(target, s) }
}

pub fn MultiTexCoord1dv(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord1dv(target, v.as_ptr()) }
}

pub fn MultiTexCoord1f(target: GLenum, s: f32) {
    unsafe { glu_sys::glMultiTexCoord1f(target, s) }
}

pub fn MultiTexCoord1fv(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord1fv(target, v.as_ptr()) }
}

pub fn MultiTexCoord1i(target: GLenum, s: i32) {
    unsafe { glu_sys::glMultiTexCoord1i(target, s) }
}

pub fn MultiTexCoord1iv(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord1iv(target, v.as_ptr()) }
}

pub fn MultiTexCoord1s(target: GLenum, s: i16) {
    unsafe { glu_sys::glMultiTexCoord1s(target, s) }
}

pub fn MultiTexCoord1sv(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord1sv(target, v.as_ptr()) }
}

pub fn MultiTexCoord2d(target: GLenum, s: f64, t: f64) {
    unsafe { glu_sys::glMultiTexCoord2d(target, s, t) }
}

pub fn MultiTexCoord2dv(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord2dv(target, v.as_ptr()) }
}

pub fn MultiTexCoord2f(target: GLenum, s: f32, t: f32) {
    unsafe { glu_sys::glMultiTexCoord2f(target, s, t) }
}

pub fn MultiTexCoord2fv(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord2fv(target, v.as_ptr()) }
}

pub fn MultiTexCoord2i(target: GLenum, s: i32, t: i32) {
    unsafe { glu_sys::glMultiTexCoord2i(target, s, t) }
}

pub fn MultiTexCoord2iv(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord2iv(target, v.as_ptr()) }
}

pub fn MultiTexCoord2s(target: GLenum, s: i16, t: i16) {
    unsafe { glu_sys::glMultiTexCoord2s(target, s, t) }
}

pub fn MultiTexCoord2sv(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord2sv(target, v.as_ptr()) }
}

pub fn MultiTexCoord3d(target: GLenum, s: f64, t: f64, r: f64) {
    unsafe { glu_sys::glMultiTexCoord3d(target, s, t, r) }
}

pub fn MultiTexCoord3dv(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord3dv(target, v.as_ptr()) }
}

pub fn MultiTexCoord3f(target: GLenum, s: f32, t: f32, r: f32) {
    unsafe { glu_sys::glMultiTexCoord3f(target, s, t, r) }
}

pub fn MultiTexCoord3fv(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord3fv(target, v.as_ptr()) }
}

pub fn MultiTexCoord3i(target: GLenum, s: i32, t: i32, r: i32) {
    unsafe { glu_sys::glMultiTexCoord3i(target, s, t, r) }
}

pub fn MultiTexCoord3iv(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord3iv(target, v.as_ptr()) }
}

pub fn MultiTexCoord3s(target: GLenum, s: i16, t: i16, r: i16) {
    unsafe { glu_sys::glMultiTexCoord3s(target, s, t, r) }
}

pub fn MultiTexCoord3sv(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord3sv(target, v.as_ptr()) }
}

pub fn MultiTexCoord4d(target: GLenum, s: f64, t: f64, r: f64, q: f64) {
    unsafe { glu_sys::glMultiTexCoord4d(target, s, t, r, q) }
}

pub fn MultiTexCoord4dv(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord4dv(target, v.as_ptr()) }
}

pub fn MultiTexCoord4f(target: GLenum, s: f32, t: f32, r: f32, q: f32) {
    unsafe { glu_sys::glMultiTexCoord4f(target, s, t, r, q) }
}

pub fn MultiTexCoord4fv(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord4fv(target, v.as_ptr()) }
}

pub fn MultiTexCoord4i(target: GLenum, s: i32, t: i32, r: i32, q: i32) {
    unsafe { glu_sys::glMultiTexCoord4i(target, s, t, r, q) }
}

pub fn MultiTexCoord4iv(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord4iv(target, v.as_ptr()) }
}

pub fn MultiTexCoord4s(target: GLenum, s: i16, t: i16, r: i16, q: i16) {
    unsafe { glu_sys::glMultiTexCoord4s(target, s, t, r, q) }
}

pub fn MultiTexCoord4sv(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord4sv(target, v.as_ptr()) }
}

pub fn LoadTransposeMatrixd(m: &[f64]) {
    unsafe { glu_sys::glLoadTransposeMatrixd(m.as_ptr()) }
}

pub fn LoadTransposeMatrixf(m: &[f32]) {
    unsafe { glu_sys::glLoadTransposeMatrixf(m.as_ptr()) }
}

pub fn MultTransposeMatrixd(m: &[f64]) {
    unsafe { glu_sys::glMultTransposeMatrixd(m.as_ptr()) }
}

pub fn MultTransposeMatrixf(m: &[f32]) {
    unsafe { glu_sys::glMultTransposeMatrixf(m.as_ptr()) }
}

pub fn SampleCoverage(value: f32, invert: bool) {
    unsafe { glu_sys::glSampleCoverage(value, invert as u8) }
}

pub fn ActiveTextureARB(texture: GLenum) {
    unsafe { glu_sys::glActiveTextureARB(texture) }
}

pub fn ClientActiveTextureARB(texture: GLenum) {
    unsafe { glu_sys::glClientActiveTextureARB(texture) }
}

pub fn MultiTexCoord1dARB(target: GLenum, s: f64) {
    unsafe { glu_sys::glMultiTexCoord1dARB(target, s) }
}

pub fn MultiTexCoord1dvARB(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord1dvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord1fARB(target: GLenum, s: f32) {
    unsafe { glu_sys::glMultiTexCoord1fARB(target, s) }
}

pub fn MultiTexCoord1fvARB(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord1fvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord1iARB(target: GLenum, s: i32) {
    unsafe { glu_sys::glMultiTexCoord1iARB(target, s) }
}

pub fn MultiTexCoord1ivARB(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord1ivARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord1sARB(target: GLenum, s: i16) {
    unsafe { glu_sys::glMultiTexCoord1sARB(target, s) }
}

pub fn MultiTexCoord1svARB(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord1svARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord2dARB(target: GLenum, s: f64, t: f64) {
    unsafe { glu_sys::glMultiTexCoord2dARB(target, s, t) }
}

pub fn MultiTexCoord2dvARB(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord2dvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord2fARB(target: GLenum, s: f32, t: f32) {
    unsafe { glu_sys::glMultiTexCoord2fARB(target, s, t) }
}

pub fn MultiTexCoord2fvARB(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord2fvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord2iARB(target: GLenum, s: i32, t: i32) {
    unsafe { glu_sys::glMultiTexCoord2iARB(target, s, t) }
}

pub fn MultiTexCoord2ivARB(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord2ivARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord2sARB(target: GLenum, s: i16, t: i16) {
    unsafe { glu_sys::glMultiTexCoord2sARB(target, s, t) }
}

pub fn MultiTexCoord2svARB(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord2svARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord3dARB(target: GLenum, s: f64, t: f64, r: f64) {
    unsafe { glu_sys::glMultiTexCoord3dARB(target, s, t, r) }
}

pub fn MultiTexCoord3dvARB(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord3dvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord3fARB(target: GLenum, s: f32, t: f32, r: f32) {
    unsafe { glu_sys::glMultiTexCoord3fARB(target, s, t, r) }
}

pub fn MultiTexCoord3fvARB(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord3fvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord3iARB(target: GLenum, s: i32, t: i32, r: i32) {
    unsafe { glu_sys::glMultiTexCoord3iARB(target, s, t, r) }
}

pub fn MultiTexCoord3ivARB(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord3ivARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord3sARB(target: GLenum, s: i16, t: i16, r: i16) {
    unsafe { glu_sys::glMultiTexCoord3sARB(target, s, t, r) }
}

pub fn MultiTexCoord3svARB(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord3svARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord4dARB(target: GLenum, s: f64, t: f64, r: f64, q: f64) {
    unsafe { glu_sys::glMultiTexCoord4dARB(target, s, t, r, q) }
}

pub fn MultiTexCoord4dvARB(target: GLenum, v: &[f64]) {
    unsafe { glu_sys::glMultiTexCoord4dvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord4fARB(target: GLenum, s: f32, t: f32, r: f32, q: f32) {
    unsafe { glu_sys::glMultiTexCoord4fARB(target, s, t, r, q) }
}

pub fn MultiTexCoord4fvARB(target: GLenum, v: &[f32]) {
    unsafe { glu_sys::glMultiTexCoord4fvARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord4iARB(target: GLenum, s: i32, t: i32, r: i32, q: i32) {
    unsafe { glu_sys::glMultiTexCoord4iARB(target, s, t, r, q) }
}

pub fn MultiTexCoord4ivARB(target: GLenum, v: &[i32]) {
    unsafe { glu_sys::glMultiTexCoord4ivARB(target, v.as_ptr()) }
}

pub fn MultiTexCoord4sARB(target: GLenum, s: i16, t: i16, r: i16, q: i16) {
    unsafe { glu_sys::glMultiTexCoord4sARB(target, s, t, r, q) }
}

pub fn MultiTexCoord4svARB(target: GLenum, v: &[i16]) {
    unsafe { glu_sys::glMultiTexCoord4svARB(target, v.as_ptr()) }
}

pub fn BlendEquationSeparateATI(modeRGB: GLenum, modeA: GLenum) {
    unsafe { glu_sys::glBlendEquationSeparateATI(modeRGB, modeA) }
}
