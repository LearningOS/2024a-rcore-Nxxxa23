///The max size of queue
const QUEUEMAXSIZE: usize = 20;

///queue
pub struct Queue<T>
{
    ///head
    head: usize,

    ///tail
    tail: usize,

    ///size
    size: usize,

    ///queue
    pub queue: [Option<T>; QUEUEMAXSIZE]
}

impl<T> Queue<T>
{
    ///new
    pub const fn new() -> Queue<T>
    {
        return Queue
        {
            head: 0,
            tail: 0,
            size: 0,
            queue: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]
        };
    }

    ///enqueue
    pub fn enqueue(&mut self, item: T)
    {
        self.queue[self.tail] = Some(item);
        self.tail = (self.tail + 1) % QUEUEMAXSIZE;
        self.size += 1;
    }

    ///dequeue
    pub fn dequeue(&mut self)
    {
        if self.is_empty()
        {
            return;
        }
        self.head = (self.head + 1) % QUEUEMAXSIZE;
        self.size -= 1;
    }

    ///peek_head
    pub fn peek_head(&mut self) -> Option<&mut T>
    {
        if self.is_empty()
        {
            return None;
        }
        return self.queue[self.head].as_mut();
    }

    ///check empty
    pub fn is_empty(&self) -> bool
    {
        return self.size == 0;
    }

    ///check full
    pub fn is_full(&self) -> bool
    {
        return self.size == QUEUEMAXSIZE;
    }

    pub fn dequeue_to_tail(&mut self)
    {
        if self.is_empty() || self.is_full() || self.size == 1
        {
            return;
        }
        self.queue[self.tail] = self.queue[self.head].take();
        self.head = (self.head + 1) % QUEUEMAXSIZE;
        self.tail = (self.tail + 1) % QUEUEMAXSIZE;
    }
}