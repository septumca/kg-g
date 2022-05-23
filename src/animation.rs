use macroquad::prelude::*;


const ANIMATION_THRESHOLD: f32 = 0.07;

struct Frames {
  list: Vec<Rect>,
  act: usize,
  should_loop: bool,
}

impl Frames {
  pub fn next_frame(&mut self) {
    self.act = if self.should_loop {
      (self.act + 1) % self.list.len()
    } else if self.act < self.list.len() - 1 {
      self.act + 1
    } else {
      self.act
    }
  }

  pub fn act_frame(&self) -> Rect {
    self.list[self.act]
  }

  pub fn is_finished(&self) -> bool {
    !self.should_loop && self.act == self.list.len() - 1
  }
}

struct Time {
  act: f32,
  threshold: f32,
}

impl Time {
  pub fn update(&mut self, delta: f32) -> bool {
    let updated_time = self.act + delta;
    let over_threshold = updated_time > self.threshold;

    self.act = if over_threshold { 0. } else { updated_time };
    over_threshold
  }
}

pub struct Animation {
  frames: Frames,
  time: Time,
}

impl Animation {
  pub fn new(frames_list: Vec<Rect>, should_loop: bool) -> Self {
    Self {
      frames: Frames {
        list: frames_list,
        act: 0,
        should_loop,
      },
      time: Time {
        act: 0.,
        threshold: ANIMATION_THRESHOLD,
      }
    }
  }

  pub fn get_act_frame(&self) -> Rect {
    self.frames.act_frame()
  }

  pub fn update(&mut self, delta: f32) {
    if self.time.update(delta) {
      self.time.act = 0.;
      self.frames.next_frame();
    }
  }
}

mod tests {
  #[cfg(test)]
  mod frames {
    use super::super::*;

    fn create(should_loop: bool) -> Frames {
      let r1 = Rect::new(0., 0., 16., 16.);
      let r2 = Rect::new(16., 0., 16., 16.);
      let r3 = Rect::new(32., 0., 16., 16.);
      Frames {
        list: vec![r1, r2, r3],
        act: 0,
        should_loop
      }
    }

    #[test]
    fn act() {
      let frames = create(true);
      assert_eq!(frames.act_frame(), frames.list[0]);
    }

    #[test]
    fn is_finished() {
      let mut frames = create(false);
      frames.next_frame();
      assert_eq!(frames.is_finished(), false);
      frames.next_frame();
      assert_eq!(frames.is_finished(), true);
      frames.next_frame();
      assert_eq!(frames.is_finished(), true);
    }

    #[test]
    fn next_loop() {
      let mut frames = create(true);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[1]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[0]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[1]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[0]);
    }

    #[test]
    fn next_not_loop() {
      let mut frames = create(false);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[1]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);

      frames.next_frame();
      assert_eq!(frames.act_frame(), frames.list[2]);
    }
  }

  #[cfg(test)]
  mod time {
    use super::super::*;

    fn create() -> Time {
      Time {
        act: 0.,
        threshold: 0.5,
      }
    }

    #[test]
    fn update() {
      let mut time = create();
      time.update(0.1);
      assert_eq!(time.act, 0.1);
      time.update(0.1);
      assert_eq!(time.act, 0.2);
      time.update(0.1);
      assert_eq!(time.act, 0.3);
    }

    #[test]
    fn update_over() {
      let mut time = create();
      time.update(0.4);
      assert_eq!(time.act, 0.4);
      time.update(0.1);
      assert_eq!(time.act, 0.5);
      time.update(0.1);
      assert_eq!(time.act, 0.);
    }
  }

  #[cfg(test)]
  mod animation {
    use super::super::*;

    #[test]
    fn update_looping() {
      let r1 = Rect::new(0., 0., 16., 16.);
      let r2 = Rect::new(16., 0., 16., 16.);
      let r3 = Rect::new(32., 0., 16., 16.);
      let mut anim = Animation::new(vec![r1, r2, r3], true);

      anim.update(ANIMATION_THRESHOLD - 0.01);
      assert_eq!(anim.get_act_frame(), r1);
      anim.update(0.02);
      assert_eq!(anim.get_act_frame(), r2);
      anim.update(ANIMATION_THRESHOLD + 0.01);
      assert_eq!(anim.get_act_frame(), r3);
      anim.update(0.001);
      assert_eq!(anim.get_act_frame(), r3);
      anim.update(0.005);
      assert_eq!(anim.get_act_frame(), r3);
      anim.update(ANIMATION_THRESHOLD);
      assert_eq!(anim.get_act_frame(), r1);
    }

    #[test]
    fn update_not_looping() {
      let r1 = Rect::new(0., 0., 16., 16.);
      let r2 = Rect::new(16., 0., 16., 16.);
      let mut anim = Animation::new(vec![r1, r2], false);

      anim.update(ANIMATION_THRESHOLD - 0.01);
      assert_eq!(anim.get_act_frame(), r1);
      anim.update(0.02);
      assert_eq!(anim.get_act_frame(), r2);
      anim.update(ANIMATION_THRESHOLD + 0.01);
      assert_eq!(anim.get_act_frame(), r2);
      anim.update(0.001);
      assert_eq!(anim.get_act_frame(), r2);
      anim.update(0.005);
      assert_eq!(anim.get_act_frame(), r2);
      anim.update(ANIMATION_THRESHOLD);
      assert_eq!(anim.get_act_frame(), r2);
    }
  }
}