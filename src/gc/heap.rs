/*

Copyright 2024 Yazalde Filimone <yazaldefilimon@gmail.com>

*/

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Object {
  pub value: i32,
  pub marked: bool,
}

#[derive(Debug)]
pub struct Heap {
  objects: HashMap<usize, Arc<Mutex<Object>>>,
  next_id: usize,
}

impl Heap {
  pub fn new() -> Self {
    Heap { objects: HashMap::new(), next_id: 0 }
  }

  pub fn allocate(&mut self, value: i32) -> usize {
    let id = self.next_id;
    self.next_id += 1;
    let object = Arc::new(Mutex::new(Object { value, marked: false }));
    self.objects.insert(id, object);
    id
  }

  pub fn get(&self, id: usize) -> Option<Arc<Mutex<Object>>> {
    self.objects.get(&id).cloned()
  }

  pub fn mark(&self, id: usize) {
    if let Some(object) = self.objects.get(&id) {
      let mut object = object.lock().unwrap();
      if !object.marked {
        object.marked = true;
        // todo: mark recursively objects referenced here, if any
      }
    }
  }

  pub fn sweep(&mut self) {
    self.objects.retain(|_, object| {
      let mut object = object.lock().unwrap();
      if object.marked {
        object.marked = false;
        return true;
      }
      return false;
    });
  }

  pub fn collect_garbage(&mut self, roots: &HashSet<usize>) {
    // Marcar a partir das raízes
    for &root in roots {
      self.mark(root);
    }

    // Limpar objetos não marcados
    self.sweep();
  }
}
