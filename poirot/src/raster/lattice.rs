use super::ComCanvas;
use ducat::core::entity::rect::Rect;
use std::cell::RefCell;
use std::rc::Rc;

const B_HEIGHT: usize = 40;
const G_HEIGHT: usize = 10;

#[derive(Debug)]
pub struct Lattice {
    pub text: String,
    pub rect: Option<Rect>,
    pub sub_height: usize,
    pub h: usize,
    pub u: usize,
    pub d: usize,
    pub image_width: usize,
    pub image_height: usize,
    pub sub_list: Option<Rc<RefCell<Vec<Lattice>>>>,
}

impl Lattice {
    pub fn new(text: String) -> Self {
        Lattice {
            text: text.clone(),
            rect: None,
            sub_height: 0,
            h: 0,
            u: 0,
            d: 0,
            image_width: 0,
            image_height: 0,
            sub_list: None,
        }
    }

    pub fn add_sub(&mut self, next: Lattice) -> usize {
        match self.sub_list {
            Some(ref x) => {
                let sl = &mut x.borrow_mut();
                let count = sl.len();
                sl.push(next);
                return count;
            }
            None => {
                let mut sl = Vec::new();
                sl.push(next);
                self.sub_list = Some(Rc::new(RefCell::new(sl)));
                return 0;
            }
        }
    }

    pub fn add_sub_list(&mut self, text: String) -> usize {
        let sn = Lattice {
            text: text.clone(),
            rect: None,
            sub_height: 0,
            h: 0,
            u: 0,
            d: 0,
            image_width: 0,
            image_height: 0,
            sub_list: None,
        };

        match self.sub_list {
            Some(ref x) => {
                let sl = &mut x.borrow_mut();
                let count = sl.len();
                sl.push(sn);
                return count;
            }
            None => {
                let mut sl = Vec::new();
                sl.push(sn);
                self.sub_list = Some(Rc::new(RefCell::new(sl)));
                return 0;
            }
        }
    }

    pub fn calc_box_height(&mut self) -> usize {
        match self.sub_list {
            Some(ref x) => {
                let sl = &mut x.borrow_mut();
                let count = sl.len();
                if count == 1 {
                    self.sub_height = 40;
                    return sl[0].calc_box_height();
                }

                let mut height = 0;
                for i in 0..count {
                    if let Some(child) = sl.get_mut(i) {
                        let cur_height = child.calc_box_height();
                        height = height + cur_height;
                    }
                }

                height = height + 10 * (count - 1);
                self.sub_height = height;
                return height;
            }
            None => {
                self.sub_height = 40;
                return 40;
            }
        }
    }

    /*
     node topo:
                -----       *****
                            * d *
                  u         *****
                          *
                        *
                ***** *
                * b *
                ***** *
              *         *
            *             *
    ***** *       d         *****
    * a *                   * c *
    ***** *     -----       *****
            *
              *
                *****
                * c *
                *****

    b: text height, 40
    g: text gap, 10
    h: neighbour child total height
    u: distance from node top edge to top most child branch
    d: distance from node bottom edge to bottom most child branch
    cs: first child of a node
    ci: child of a node with index i
    cm: child of a node except the first and last child
    ce: last child of a node

    for node without child:
    h = b
    u = 0
    d = 0

    for node with one child:
    h = b
    u = ([u] in cs)
    d = ([d] in cs)

    for node with two child:
    h = 2b + g + ([d] in cs) + ([u] in ce)
    u = (h/2) - (b/2) + ([u] in cs)
    d = (h/2) - (b/2) + ([d] in ce)

    for other node:
    h = b * count + g * (count - 1) + ([d] in cs) + ([u + d] in cm) + ([u] in ce)
    u = (h/2) - (b/2) + ([u] in cs)
    d = (h/2) - (b/2) + ([d] in ce)
    */
    pub fn calc_top_bottom(&mut self) -> usize {
        match self.sub_list {
            Some(ref x) => {
                let sl = &mut x.borrow_mut();
                let count = sl.len();

                if count == 1 {
                    let tmp_height = sl[0].calc_top_bottom();
                    self.h = B_HEIGHT;
                    self.u = sl[0].u;
                    self.d = sl[0].d;

                    return tmp_height;
                }

                for i in 0..count {
                    if let Some(child) = sl.get_mut(i) {
                        child.calc_top_bottom();
                    }
                }

                if count == 2 {
                    self.h = 2 * B_HEIGHT + G_HEIGHT + sl[0].d + sl[1].u;
                } else {
                    self.h = count * B_HEIGHT + (count - 1) * G_HEIGHT + sl[0].d + sl[count - 1].u;
                    for i in 1..(count - 1) {
                        if let Some(child) = sl.get_mut(i) {
                            self.h += child.u + child.d;
                        }
                    }
                }

                self.u = (self.h / 2) - (B_HEIGHT / 2) + sl[0].u;
                self.d = (self.h / 2) - (B_HEIGHT / 2) + sl[count - 1].d;

                return self.h;
            }
            None => {
                self.h = B_HEIGHT;
                self.u = 0;
                self.d = 0;

                return B_HEIGHT;
            }
        }
    }

    pub fn draw_start(&mut self, cc: &mut ComCanvas, w0: usize, h0: usize) {
        self.calc_box_height();
        self.calc_top_bottom();
        self.draw_node(cc, w0, h0, None);
    }

    pub fn draw_node(&self, cc: &mut ComCanvas, w0: usize, h0: usize, prefix: Option<(i32, i32)>) {
        let off = (w0 as i32, h0 as i32);
        let r = cc.draw_rect(off, 4, &self.text);

        let left = r.left();
        let right = r.right();
        let top = r.top();
        let bottom = r.bottom();

        let left_point = (left, (top + bottom) / 2);
        let right_end = (right, (top + bottom) / 2);

        match prefix {
            Some(start) => {
                cc.draw_con_diag(start, left_point);
            }
            None => {}
        }

        match self.sub_list {
            Some(ref x) => {
                let h = self.h;
                let sl = &mut x.borrow_mut();
                let count = sl.len();

                let w1 = right as usize + 50;
                let mut h1 = ((bottom + top) / 2) as i32;

                if count >= 2 {
                    h1 = h1 - (h / 2) as i32;

                    let mut offset = 0;

                    for i in 0..count {
                        if let Some(child) = sl.get(i) {
                            if offset > 0 {
                                offset += G_HEIGHT + child.u;
                            }
                            let cur_height = h1 + offset as i32;
                            child.draw_node(cc, w1, cur_height as usize, Some(right_end));

                            offset += B_HEIGHT + child.d;
                        }
                    }
                } else {
                    sl[0].draw_node(cc, w1, top as usize, Some(right_end));
                }

                return;
            }
            None => {
                return;
            }
        }
    }
}
