;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader utilities
;;;

;;;
;;; loader
;;;
(mu:intern prelude "load"
   (:lambda (path verbose)
     (:if (core:stringp path)
         ((:lambda (ifs)
            (:if (core:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (core:%format :t ";;; loading ~A~%" `(,path))
                         ())
                    ((:lambda (status)
                       (:if (mu:eq core:%eof% status)
                            :t
                            ()))
                     (mu:fix
                        (:lambda (loop)
                           (:if (mu:eq core:%eof% loop)
                                core:%eof%
                                (core:with-exception
                                    (:lambda (exception)
                                      (core:%exceptionf
                                       mu:error-output
                                       "loader exception: ~A on ~A by ~S, ~A~%"
                                       ()
                                       exception))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (mu:eq core:%eof% form)
                                               core:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (core:%format :t "~A~%" `(,value))
                                                       ())
                                                  (core:null loop))
                                                (mu:eval (core:compile form)))))
                                       (mu:read ifs () core:%eof%))))))
                       ()))))
                  (core:error "load:  cannot open input file, ~A~%" `(,ifs))))
            (mu:open :file :input path :t))
          (core:error "load: not a file designator, ~A~%" `(,path)))))
