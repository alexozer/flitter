open Base
open Timer_types

let test () =
  let pb = Some {
      attempt = 2500;
      splits = [|
        {title = "Green"; time = Some 3000; is_gold = false};
        {title = "Apricot"; time = Some 5000; is_gold = false};
        {title = "Blue"; time = Some 8000; is_gold = false};
      |];
    } in

  let timer = {
    title = "Super Monkey Ball 2: Monkeyed Ball";
    category = "Story Mode All Levels";
    attempts = 3000;
    completed = 40;

    split_names = [|
      "Green";
      "Apricot";
      "Blue";
    |];

    golds = [|
      {title = "Green"; duration = Some 2000};
      {title = "Apricot"; duration = Some 2000};
      {title = "Blue"; duration = Some 2000};
    |];

    pb = pb;
    comparison = pb;

    history = [];

    start_time = Unix.gettimeofday ();
    state = Timing;
    splits = Array.of_list [Some 1500; None; None];
    curr_split = 2;
  } in

  let%lwt event_loop = Event_loop.make timer in
  Event_loop.loop event_loop

let run () =
  Lwt_main.run (test ())