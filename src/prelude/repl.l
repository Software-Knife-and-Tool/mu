;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl
;;;
(libcore:intern :prelude "repl"
   (:lambda ()
     (libcore:fix
       (:lambda (loop)
         (prelude:with-exception
             (:lambda (ex)
               (prelude:break ex)
               (prelude:null loop))
           (:lambda ()
             (prelude:format :t "prelude> " ())
             (libcore:flush libcore:std-out)
             ((:lambda (form)
                (:if (libcore:eq form prelude:%eof%)
                     loop
                     ((:lambda (value)
                        (prelude:format :t "[~A] ~S~%" `(,(prelude:type-of value) ,value))
                        (prelude:null loop))
                      (libcore:eval (prelude:compile form)))))
              (prelude:read :t () prelude:%eof%)))))
       ())))
