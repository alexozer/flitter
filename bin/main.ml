open Splits

let () =
  (* let force = function
     | Some x -> x
     | None -> raise @@ Failure "Unable to decode time"
     in *)

  let run = {
    game = {
      title = "Super Monkey Ball 2: Monkeyed Ball";
      category = "Story Mode All Levels";
      attempts = 3000;
      completed_runs = 40;

      split_names = [|
        "Green";
        "Apricot";
        "Blue";
      |];

      golds = Some [|
          3000;
          3000;
          3000;
        |];
    };

    comparison = Some [|
        4000;
        6000;
        8500;
      |];

    start_time = Unix.gettimeofday ();
    state = Timing;
    splits = Array.make 3 None;
    curr_split = 1;
  } in

  let rec refresh disp =
    Display.draw disp run;
    Unix.sleepf (1. /. 60.);
    refresh disp
  in
  refresh (Display.make ())