open Timer_types

let timer =
  let pb = Some {
      attempt = 2000;
      splits = [|
        {title = "Green"; time = Some 2; is_gold = false};
        {title = "Apricot"; time = Some 4; is_gold = false};
        {title = "Blue"; time = Some 6; is_gold = false};
        {title = "Brown"; time = Some 8; is_gold = false};
        {title = "Pink"; time = Some 10; is_gold = false};
        {title = "White"; time = Some 12; is_gold = false};
        {title = "Teal"; time = Some 14; is_gold = false};
        {title = "Orange"; time = Some 16; is_gold = false};
        {title = "Gold"; time = Some 18; is_gold = false};
        {title = "Black"; time = Some 20; is_gold = false};
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
      {title = "Green"; duration = Some 1};
      {title = "Apricot"; duration = Some 1};
      {title = "Blue"; duration = Some 1};
      {title = "Brown"; duration = Some 1};
      {title = "Pink"; duration = Some 1};
      {title = "White"; duration = Some 1};
      {title = "Teal"; duration = Some 1};
      {title = "Orange"; duration = Some 1};
      {title = "Gold"; duration = Some 1};
      {title = "Black"; duration = Some 1};
    |];

    comparison = pb;
    history = [];
    state = Idle;
  }
