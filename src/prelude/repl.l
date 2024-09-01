;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl
;;;
(mu:intern prelude "repl"
   (:lambda ()
     (mu:fix
       (:lambda (loop)
         (core:with-exception
             (:lambda (ex)
               (prelude:break ex)
               (core:null loop))
           (:lambda ()
             (core:format :t "prelude> " ())
             (mu:flush mu:*standard-output*)
             ((:lambda (form)
                (:if (mu:eq form core:%eof%)
                     loop
                     ((:lambda (value)
                        (core:format :t "[~A] ~S~%" `(,(core:type-of value) ,value))
                        (core:null loop))
                      (mu:eval (core:compile form)))))
              (core:read mu:*standard-input* () core:%eof%)))))
       ())))

#|
(defun repl (ns)
  (errorp-unless namespacep ns "repl: not a namespace")
  (flet ((repl (repl)
          (unwind-protect
              (lambda (ex)  ;; catch exception and break
                (print-except ex "core:repl")
                (break ex)
                (repl repl))
              (lambda ()    ;; main
                (flet ((loop (loop form)
                        (if (eofp t)
                            (exit 0)
                            (if (keywordp form)
                                (cond
                                  ((eq form :h)
                                   (format t ";;; help: lib ~A~%:l - load file~%:m - expand macro~%" core::version))
                                  ((eq form :m) (format t "~A~%" (macroexpand-1 (read t t ()))))
                                  ((eq form :l)
                                   ((lambda (path)
                                      (if (stringp path)
                                          (load path t ())
                                          (format t ":l ~A is not a file path~%" path)))
                                    (read t t ())))
                                  (t ((lambda ()
                                        (write form () ())
                                        (terpri ())))))
                                  (format t "~A~%" (eval (compile form)))))
                        (format t "~A> " (ns-name ns))
                        (loop loop (read t t ()))))
                  (format t "~A> " (ns-name ns))
                  (loop loop (read t t ())))))))
    (format t ";;; Eko Lisp ~A (core:repl) :h for help~%" core::eko-version)
    (repl repl)))
|#
