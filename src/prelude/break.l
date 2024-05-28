;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
#|
(crux:intern prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (crux:boundp 'prelude:%debug-symbols%)
           ()
           (crux:intern prelude "%debug-symbols%" (crux:ns-syms :list prelude)))
      (crux:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (crux:eq (crux:symbol-value symbol) value)
                        symbol
                        (crux:cdr list)))
                (crux:car list))
               list))
       prelude:%debug-symbols%)))

(crux:intern prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))
|#

;;;
;;; break loop
;;;
(crux:intern prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (crux:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (crux:eq form prelude:%eof%)
                loop
                (:if (crux:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (crux:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (crux:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (crux:eq form :x)
                               (crux:exit 0)
                               (:if (crux:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf crux:*standard-output* "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (crux:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (crux:flush crux:*standard-output*)
            (prelude:read :t () prelude:%eof%)))))
     ())))
