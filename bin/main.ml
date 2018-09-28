open Splits

let () =
  let force = function
    | Some x -> x
    | None -> raise @@ Failure "Unable to decode time"
  in

  let run = {
    game_info = {
      game = "Super Monkey Ball 2: Monkeyed Ball";
      category = "Story Mode All Levels";
      attempts = 3000;
      completed_runs = 40;
    };

    comparison = [|
      {
        title = "Green";
        time = Duration.of_string "2:11" |> force;
        is_gold_split = false;
      };
      {
        title = "Apricot";
        time = Duration.of_string "4:31.000" |> force;
        is_gold_split = false;
      };
      {
        title = "Blue";
        time = Duration.of_string "6:35.032" |> force;
        is_gold_split = true;
      };
    |];

    golds = [|
      {
        title = "Green";
        duration = Duration.of_string "2:10" |> force;
      };
      {
        title = "Apricot";
        duration = Duration.of_string "2:18" |> force;
      };
      {
        title = "Blue";
        duration = Duration.of_string "2:04.032" |> force;
      };
    |];

    start_time = Unix.gettimeofday ();
    state = Timing;

    splits = [];
  } in

  let rec refresh disp =
    Display.draw disp run;
    Unix.sleepf (1. /. 60.);
    refresh disp
  in
  refresh (Display.make ())