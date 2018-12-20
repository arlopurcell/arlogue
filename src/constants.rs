pub const MONSTER_SPELLBOOK: &'static str = "noop;
left:
store #left d;
move d;
return;

right:
store #right d;
move d;
return;

up:
store #up d;
move d;
return;

down:
store #down d;
move d;
return;

wait: return;

attack_left:
store #left d;
move_cursor d;
store 5 a;
damage a;
return;

attack_right:
store #right d;
move_cursor d;
store 5 a;
damage a;
return;

attack_up:
store #up d;
move_cursor d;
store 5 a;
damage a;
return;

attack_down:
store #down d;
move_cursor d;
store 5 a;
damage a;
return;

";
