;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
(mu:intern :prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (mu:boundp 'prelude:%debug-symbols%)
           ()
           (mu:intern :prelude "%debug-symbols%" (mu:ns-syms :list :prelude)))
      (mu:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (mu:eq (mu:sy-val symbol) value)
                        symbol
                        (mu:cdr list)))
                (mu:car list))
               list))
       prelude:%debug-symbols%)))

(mu:intern :prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))

;;;
;;; break loop
;;;
(mu:intern :prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (mu:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (mu:eq form prelude:%eof%)
                loop
                (:if (mu:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (mu:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (mu:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (mu:eq form :x)
                               (mu:exit 0)
                               (:if (mu:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf mu:std-out "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (mu:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (mu:flush mu:std-out)
            (prelude:read :t () prelude:%eof%)))))
     ())))
