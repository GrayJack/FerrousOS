// TODO Improve this, as this will go boom very quickly
// Implement a reuse mechanism. Shouldnt be hard using Option<T>
// Maybe move to a more appropriate module

use core::default::Default;

pub struct Queue<'a, T: Copy + Default + Sized> {
    array: &'a mut [T],
    front: usize,
    rear: isize,
    max: isize
}

impl<'a, T: Copy + Default + Sized> Queue<'a, T> {
    pub fn new(size: usize) -> Queue<'a, T> {
        //Queue {
            //array: [Default::default(); size],
            //front: 0,
            //rear: -1,
            //max: size
        //} 
        
        unimplemented!()
    }

    pub fn enqueue(&mut self, obj: &'a T) -> Result<(), &'static str> {
        if self.rear == self.max - 1 {
            Err("Queue overflow")
        } else {
            self.rear += 1;
            self.array[self.rear as usize] = *obj;

            Ok(())
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.front == self.rear as usize + 1  {
            None
        } else {
            let item = self.array[self.front];
            self.front += 1;
            Some(item)
        }
    }
}
