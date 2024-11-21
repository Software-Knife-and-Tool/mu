;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl
;;;
(mu:intern repl "repl"
   (:lambda ()
     (mu:fix
       (:lambda (loop)
         (core:with-exception
             (:lambda (ex)
               (repl:break ex)
               (core:null loop))
           (:lambda ()
             (core:%format :t "prelude> " ())
             (mu:flush mu:*standard-output*)
             ((:lambda (form)
                (:if (mu:eq form core:%eof%)
                     loop
                     ((:lambda (value)
                        (core:%format :t "[~A] ~S~%" `(,(core:type-of value) ,value))
                        (core:null loop))
                      (mu:eval (core:compile form)))))
              (core:read mu:*standard-input* () core:%eof%)))))
       ())))
