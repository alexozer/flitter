open Timer_types

let timer =
  let pb = Some {
      attempt = 2500;
      splits = [|
        {title = "Green"; time = Some 3000; is_gold = false};
        {title = "Apricot"; time = Some 5000; is_gold = false};
        {title = "Blue"; time = Some 8000; is_gold = false};
      |];
    } in

  {
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

    comparison = pb;
    history = [];
    state = Timing ([|Some 1500|], Unix.gettimeofday ())
  }
