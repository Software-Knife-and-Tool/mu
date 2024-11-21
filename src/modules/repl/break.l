;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; break loop
;;;
(mu:intern repl "break"
   (:lambda (exception)
     (core:%format :t ";;; entering break loop with exception~%" ())
     (core:%exceptionf :t ";;; ~A on ~S by ~S, ~A~%" () exception)
     (core:%format :t ";;; :h for commands~%" ())
     (mu:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (mu:eq form core:%eof%)
                loop
                (:if (mu:eq form :h)
                     ((:lambda ()
                        (core:%format :t "break help:~%" ())
                        (core:%format :t ":d - describe exception value~%" ())
                        (core:%format :t ":e - print the full exception~%" ())
                        (core:%format :t ":r - return from break~%" ())
                        (core:%format :t ":x - exit process~%" ())
                        (core:null loop)))
                     (:if (mu:eq form :d)
                          ((:lambda (value)
                             ;;; (prelude:describe (mu:cdr value) :t)
                             (core:null loop))
                           (core:%exception-prop :value except))
                          (:if (mu:eq form :x)
                               (mu:exit 0)
                               (:if (mu:eq form :e)
                                    ((:lambda ()
                                       (core:%exceptionf mu:*standard-output* "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (core:null loop)))
                                    (:if (mu:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (core:%format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (core:null loop))))))))))
         ((:lambda ()
            (core:%format :t "prelude:debug> " ())
            (mu:flush mu:*standard-output*)
            (core:read mu:*standard-input* () core:%eof%)))))
     ())))
