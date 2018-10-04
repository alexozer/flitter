type split = {
  title : string;
  time : Duration.t option;
  is_gold: bool;
}

type gold = {
  title : string;
  duration : Duration.t option;
}

type archived_run = {
  attempt : int;
  splits : split array;
}

type game = {
  title : string;
  category : string;
  attempts : int;
  completed : int;

  split_names : string array;
  pb : archived_run option;
  golds : gold array;
  history : archived_run list;
}

type timer_state = Idle | Timing | Paused of float | Done

type run = {
  game : game;
  comparison : archived_run;

  start_time : float;
  state : timer_state;
  splits : Duration.t option array;
  curr_split : int;
}