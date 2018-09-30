(* open Splits

   let () =
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
          2000;
          2000;
          2000;
        |];
    };

    comparison = Some [|
        3000;
        5000;
        8000;
      |];

    start_time = Unix.gettimeofday ();
    state = Timing;
    splits = Array.of_list [Some 1500; None; None];
    curr_split = 2;
   } in

   let rec refresh disp =
    Display.draw disp run;
    Unix.sleepf (1. /. 60.);
    refresh disp
   in
   refresh (Display.make ()) *)

let show_hotkeys () =
  let%lwt stream = Hotkeys.make_stream () in
  let rec show () =
    match%lwt Lwt_stream.get stream with
    | Some (time, keypress) ->
      let%lwt () = Lwt_io.printl @@ "Got " ^ keypress ^ " at time " ^ Float.to_string time in
      show ()
    | None -> Lwt.return ()
  in
  show ()

let run () =
  Lwt_main.run (show_hotkeys ())