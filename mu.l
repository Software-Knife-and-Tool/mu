(lib:intern () "future-loop"
  (:lambda ()
    (lib:fapply
     (:lambda ()
       (lib:fix
        (:lambda (keep-alive)
          (std:sleep 1.0)
          (lib:write (lib:hp-stat) () lib:std-out)
          (lib:write #\linefeed () lib:std-out)
          (lib:flush lib:std-out)
          (prelude:null keep-alive))
        ()))
     ())))
