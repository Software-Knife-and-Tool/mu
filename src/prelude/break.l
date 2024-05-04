;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
(lib:intern prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (lib:boundp 'prelude:%debug-symbols%)
           ()
           (lib:intern prelude "%debug-symbols%" (lib:ns-syms :list prelude)))
      (lib:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (lib:eq (lib:symbol-value symbol) value)
                        symbol
                        (lib:cdr list)))
                (lib:car list))
               list))
       prelude:%debug-symbols%)))

(lib:intern prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))

;;;
;;; break loop
;;;
(lib:intern prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (lib:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (lib:eq form prelude:%eof%)
                loop
                (:if (lib:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (lib:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (lib:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (lib:eq form :x)
                               (lib:exit 0)
                               (:if (lib:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf lib:std-out "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (lib:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (lib:flush lib:std-out)
            (prelude:read :t () prelude:%eof%)))))
     ())))
