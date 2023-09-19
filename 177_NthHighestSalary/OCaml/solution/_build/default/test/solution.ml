type employee = {
  id : int;
  name : string;
  salary : float;
}
;;

let nth_highest_salary employees n = 
  let salaries =
    employees
    |> List.map (fun e -> e.salary)
    |> List.sort_uniq compare
    |> List.rev 
    (*this defines "salary to get a list that maps the employee objects
    into a list of their salaries and then drops non-unique ones, and
    then reverses the list "*)
  in
  match List.nth_opt salaries (n - 1) with
  | Some salary -> Some salary
  | None -> None
;;


let employees = [
  {id=1; name="1"; salary=90000.};
  {id=2; name="2"; salary=60000.};
  {id=3; name="3"; salary=60000.}; (* Duplicate salary *)
  {id=4; name="4"; salary=75000.};
  {id=5; name="5"; salary=15000.};
]
;;


let () = 
  let test_n = 1 in
  match nth_highest_salary employees test_n with 
  | Some salary -> Printf.printf "The %dth highest salary is: %f\n" test_n salary
  | None -> Printf.printf "There is no %dth highest salary\n" test_n
;;