;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
(core:intern :prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (core:boundp 'prelude:%debug-symbols%)
           ()
           (core:intern :prelude "%debug-symbols%" (core:ns-syms :list :prelude)))
      (core:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (core:eq (core:sy-val symbol) value)
                        symbol
                        (core:cdr list)))
                (core:car list))
               list))
       prelude:%debug-symbols%)))

(core:intern :prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))

;;;
;;; break loop
;;;
(core:intern :prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (core:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (core:eq form prelude:%eof%)
                loop
                (:if (core:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (core:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (core:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (core:eq form :x)
                               (core:exit 0)
                               (:if (core:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf core:std-out "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (core:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (core:flush core:std-out)
            (prelude:read :t () prelude:%eof%)))))
     ())))
