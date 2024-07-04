
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


fn addlist(l1: Option<Box<ListNode>>,l2: Option<Box<ListNode>>,list:&mut Vec<i32>) {
    let l1clone = l1.clone();
    let l2clone = l2.clone();
    if !l1.is_none() && !l2.is_none() {
        addlist(l1.unwrap().next,l2.unwrap().next,list);
        list.push(l1clone.as_deref().unwrap().val + l2clone.as_deref().unwrap().val);
    }

    else if !l1.is_none() && l2.is_none(){
        addlist(l1.unwrap().next,l2,list);
        list.push(l1clone.as_deref().unwrap().val);
    }
    
    else if l1.is_none() && !l2.is_none(){
        addlist(l1,l2.unwrap().next,list);
        list.push(l2clone.as_deref().unwrap().val);
    }
}

fn changecarry(list:&mut Vec<i32>, carry:i32, lastindex: i32){
    if lastindex==-1 {
        return
    }
    list[lastindex as usize]+= carry;
    let newcarry = list[lastindex as usize]/10;
    list[lastindex as usize] = list[lastindex as usize] % 10;
    changecarry(list, newcarry, lastindex-1);
}

fn createlist(list:&mut Vec<i32>,index:i32) -> Option<Box<ListNode>>{

    if index == list.len() as i32 {
        return None;
    }

    let mut node = Some(Box::new(ListNode::new(list[index as usize])));
    node.as_deref_mut().unwrap().next = createlist(list, index+1);
    return node;
}

fn main(){
    let mut node1 = Some(Box::new(ListNode::new(1)));
    node1.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    node1.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    let mut node2 = Some(Box::new(ListNode::new(6)));
    node2.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(7)));

    let mut list: Vec<i32>= Vec::new();

    addlist(node1,node2,&mut list);

    let ind = (list.len()-1)  as i32;
    
    changecarry(&mut list, 0, 2);

    let node = createlist(&mut list, 0);

    for i in list.iter() {
        println!("{}",i);
    }


    println!("{:?}",node.unwrap().next);
}