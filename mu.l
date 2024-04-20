(lib:intern () "future-loop"
  (:lambda ()
    (lib:defer
     :lazy
     (:lambda ()
       (lib:write (lib:hp-stat) () lib:std-out)
       (lib:flush lib:std-out))
     ())))
