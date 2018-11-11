open Timer_types

let timer =
  let pb = Some {
      attempt = 2000;
      splits = [|
        {title = "Green"; time = Some 2000; is_gold = false};
        {title = "Apricot"; time = Some 4000; is_gold = false};
        {title = "Blue"; time = Some 6000; is_gold = false};
        {title = "Brown"; time = Some 8000; is_gold = false};
        {title = "Pink"; time = Some 10000; is_gold = false};
        {title = "White"; time = Some 12000; is_gold = false};
        {title = "Teal"; time = Some 14000; is_gold = false};
        {title = "Orange"; time = Some 16000; is_gold = false};
        {title = "Gold"; time = Some 18000; is_gold = false};
        {title = "Black"; time = Some 20000; is_gold = false};
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
      "Brown";
      "Pink";
      "White";
      "Teal";
      "Orange";
      "Gold";
      "Black";
    |];

    golds = [|
      {title = "Green"; duration = Some 1000};
      {title = "Apricot"; duration = Some 1000};
      {title = "Blue"; duration = Some 1000};
      {title = "Brown"; duration = Some 1000};
      {title = "Pink"; duration = Some 1000};
      {title = "White"; duration = Some 1000};
      {title = "Teal"; duration = Some 1000};
      {title = "Orange"; duration = Some 1000};
      {title = "Gold"; duration = Some 1000};
      {title = "Black"; duration = Some 1000};
    |];

    comparison = pb;
    pb = pb;
    history = [];
    state = Idle;

    splits_file = "";
  }
