# Rastrum class
Rastrum is our interpretation of a monitor.  

Proposal of the Knight class:
```
while(1) {
  self.is_talking = false;
  rastrum.start_talking(index);
  self.is_talking = true;
  sleep(rand_int());
  rastrum.stop_talking(index);
}
```

Since Rastrum is a monitor, we are sure that only one Knight will be calling the start talking or stop talking functions.

Proposal of the Rastrum class:  
?


https://stackoverflow.com/questions/31373255/how-do-i-share-a-mutable-object-between-threads

https://rustbyexample.com/trait.html
