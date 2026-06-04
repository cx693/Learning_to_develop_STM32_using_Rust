#![allow(dead_code)]

const SIN_TAB: [i16; 256] = [
      0,   3,   6,   9,  12,  16,  19,  22,  25,  28,  31,  34,  37,  40,  43,  46,
     49,  52,  55,  58,  60,  63,  66,  68,  71,  73,  76,  78,  81,  83,  85,  88,
     90,  92,  94,  96,  98, 100, 102, 104, 106, 107, 109, 111, 112, 113, 115, 116,
    117, 118, 120, 121, 122, 122, 123, 124, 125, 125, 126, 126, 126, 127, 127, 127,
    127, 127, 127, 127, 126, 126, 126, 125, 125, 124, 123, 122, 122, 121, 120, 118,
    117, 116, 115, 113, 112, 111, 109, 107, 106, 104, 102, 100,  98,  96,  94,  92,
     90,  88,  85,  83,  81,  78,  76,  73,  71,  68,  66,  63,  60,  58,  55,  52,
     49,  46,  43,  40,  37,  34,  31,  28,  25,  22,  19,  16,  12,   9,   6,   3,
      0,  -3,  -6,  -9, -12, -16, -19, -22, -25, -28, -31, -34, -37, -40, -43, -46,
    -49, -52, -55, -58, -60, -63, -66, -68, -71, -73, -76, -78, -81, -83, -85, -88,
    -90, -92, -94, -96, -98,-100,-102,-104,-106,-107,-109,-111,-112,-113,-115,-116,
   -117,-118,-120,-121,-122,-122,-123,-124,-125,-125,-126,-126,-126,-127,-127,-127,
   -127,-127,-127,-127,-126,-126,-126,-125,-125,-124,-123,-122,-122,-121,-120,-118,
   -117,-116,-115,-113,-112,-111,-109,-107,-106,-104,-102,-100, -98, -96, -94, -92,
    -90, -88, -85, -83, -81, -78, -76, -73, -71, -68, -66, -63, -60, -58, -55, -52,
    -49, -46, -43, -40, -37, -34, -31, -28, -25, -22, -19, -16, -12,  -9,  -6,  -3,
];

fn sin_i(angle: u8) -> i16 {
    SIN_TAB[angle as usize]
}

fn cos_i(angle: u8) -> i16 {
    SIN_TAB[((angle as u16 + 64) & 0xFF) as usize]
}

fn rot(a: i16, b: i16, c: i16, s: i16) -> (i16, i16) {
    let ra = ((a as i32 * c as i32) - (b as i32 * s as i32)) >> 7;
    let rb = ((a as i32 * s as i32) + (b as i32 * c as i32)) >> 7;
    (ra as i16, rb as i16)
}

const FACE_INDICES: [[usize; 4]; 6] = [
    [0, 3, 2, 1],
    [4, 5, 6, 7],
    [0, 4, 7, 3],
    [1, 2, 6, 5],
    [0, 1, 5, 4],
    [3, 7, 6, 2],
];

const FACE_NORMALS: [[i16; 3]; 6] = [
    [   0,    0, -127],
    [   0,    0,  127],
    [-127,    0,    0],
    [ 127,    0,    0],
    [   0, -127,    0],
    [   0,  127,    0],
];

const EDGE_INDICES: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0),
    (4, 5), (5, 6), (6, 7), (7, 4),
    (0, 4), (1, 5), (2, 6), (3, 7),
];

pub struct CubeConfig {
    pub size: i16,
    pub face_colors: [u16; 6],
    pub edge_color: u16,
    pub draw_edges: bool,
    pub cx: i16,
    pub cy: i16,
    pub fov: i16,
    pub depth_offset: i16,
}

impl Default for CubeConfig {
    fn default() -> Self {
        CubeConfig {
            size: 40,
            face_colors: [0xF800, 0xFFE0, 0x07E0, 0x07FF, 0x001F, 0xF81F],
            edge_color: 0xFFFF,
            draw_edges: true,
            cx: 120,
            cy: 120,
            fov: 200,
            depth_offset: 200,
        }
    }
}

#[derive(Clone, Copy)]
struct FaceInfo {
    idx: usize,
    depth: i16,
}

pub struct Cube3D {
    pub angle_x: u8,
    pub angle_y: u8,
    pub angle_z: u8,
    pub config: CubeConfig,
    vertices: [[i16; 3]; 8],
    projected: [[i16; 2]; 8],
    visible_faces: [FaceInfo; 3],
    visible_count: usize,
}

impl Cube3D {
    pub fn new(config: CubeConfig) -> Self {
        let s = config.size;
        let vertices = [
            [-s, -s, -s],
            [ s, -s, -s],
            [ s,  s, -s],
            [-s,  s, -s],
            [-s, -s,  s],
            [ s, -s,  s],
            [ s,  s,  s],
            [-s,  s,  s],
        ];
        Cube3D {
            angle_x: 0,
            angle_y: 0,
            angle_z: 0,
            config,
            vertices,
            projected: [[0; 2]; 8],
            visible_faces: [FaceInfo { idx: 0, depth: 0 }; 3],
            visible_count: 0,
        }
    }

    pub fn rotate_and_project(&mut self) {
        let rcx = cos_i(self.angle_x);
        let rsx = sin_i(self.angle_x);
        let rcy = cos_i(self.angle_y);
        let rsy = sin_i(self.angle_y);
        let rcz = cos_i(self.angle_z);
        let rsz = sin_i(self.angle_z);

        for i in 0..8 {
            let (x, y, z) = (self.vertices[i][0], self.vertices[i][1], self.vertices[i][2]);
            let (y1, z1) = rot(y, z, rcx, rsx);
            let (x2, z2) = rot(x, z1, rcy, rsy);
            let (x3, y3) = rot(x2, y1, rcz, rsz);

            let z_depth = z2 as i32 + self.config.depth_offset as i32;
            let zd = if z_depth > 0 { z_depth } else { 1 };

            self.projected[i][0] = self.config.cx + ((x3 as i32 * self.config.fov as i32) / zd) as i16;
            self.projected[i][1] = self.config.cy + ((y3 as i32 * self.config.fov as i32) / zd) as i16;
        }

        self.visible_count = 0;

        for f in 0..6 {
            let n = &FACE_NORMALS[f];
            let (_ny1, nz1) = rot(n[1], n[2], rcx, rsx);
            let (_nx2, nz2) = rot(n[0], nz1, rcy, rsy);

            if nz2 >= 0 {
                continue;
            }

            if self.visible_count < 3 {
                let mut pos = self.visible_count;
                for j in 0..self.visible_count {
                    if nz2 > self.visible_faces[j].depth
                        || (nz2 == self.visible_faces[j].depth && f > self.visible_faces[j].idx)
                    {
                        pos = j;
                        break;
                    }
                }
                let mut k = self.visible_count;
                while k > pos {
                    self.visible_faces[k] = self.visible_faces[k - 1];
                    k -= 1;
                }
                self.visible_faces[pos] = FaceInfo { idx: f, depth: nz2 };
                self.visible_count += 1;
            }
        }
    }

    pub fn step(&mut self) {
        self.angle_x = self.angle_x.wrapping_add(1);
        self.angle_y = self.angle_y.wrapping_add(2);
        self.angle_z = self.angle_z.wrapping_add(1);
    }

    pub fn get_visible_faces(&self) -> impl Iterator<Item = (usize, u16)> + '_ {
        (0..self.visible_count).map(move |i| {
            let f = self.visible_faces[i].idx;
            (f, self.config.face_colors[f])
        })
    }

    pub fn get_face_vertices(&self, face_idx: usize) -> [[i16; 2]; 4] {
        let fi = &FACE_INDICES[face_idx];
        [self.projected[fi[0]], self.projected[fi[1]], self.projected[fi[2]], self.projected[fi[3]]]
    }

    pub fn get_edges(&self) -> &[(usize, usize); 12] {
        &EDGE_INDICES
    }

    pub fn get_bounding_box(&self) -> (u16, u16, u16, u16) {
        let mut min_x = self.projected[0][0];
        let mut max_x = min_x;
        let mut min_y = self.projected[0][1];
        let mut max_y = min_y;
        for i in 1..8 {
            let px = self.projected[i][0];
            let py = self.projected[i][1];
            if px < min_x { min_x = px; }
            if px > max_x { max_x = px; }
            if py < min_y { min_y = py; }
            if py > max_y { max_y = py; }
        }
        let pad: i16 = 2;
        let x0 = if min_x - pad >= 0 { (min_x - pad) as u16 } else { 0 };
        let y0 = if min_y - pad >= 0 { (min_y - pad) as u16 } else { 0 };
        let x1 = if max_x + pad < 240 { (max_x + pad) as u16 } else { 239 };
        let y1 = if max_y + pad < 240 { (max_y + pad) as u16 } else { 239 };
        (x0, y0, x1 - x0 + 1, y1 - y0 + 1)
    }

    pub fn get_projected(&self) -> &[[i16; 2]; 8] {
        &self.projected
    }

    pub fn get_edge_color(&self) -> u16 {
        self.config.edge_color
    }

    pub fn should_draw_edges(&self) -> bool {
        self.config.draw_edges
    }
}
