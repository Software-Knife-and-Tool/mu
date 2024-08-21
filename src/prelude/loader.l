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
    (:if (prelude:stringp path)
         ((:lambda (ifs)
            (:if (prelude:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (prelude:format :t ";;; loading ~A~%" `(,path))
                         ())
                    ((:lambda (status)
                       (:if (mu:eq prelude:%eof% status)
                            :t
                            ()))
                     (mu:fix
                        (:lambda (loop)
                           (:if (mu:eq prelude:%eof% loop)
                                prelude:%eof%
                                (prelude:with-exception
                                    (:lambda (exception)
                                      (prelude:%exceptionf
                                       mu:error-output
                                       "loader exception: ~A on ~A by ~S, ~A~%"
                                       ()
                                       exception))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (mu:eq prelude:%eof% form)
                                               prelude:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (prelude:format :t "~A~%" `(,value))
                                                       ())
                                                  (prelude:null loop))
                                                (mu:eval (prelude:compile form)))))
                                       (mu:read ifs () prelude:%eof%))))))
                       ()))))
                  (prelude:raise prelude:streamp ifs 'prelude:load "cannot open input file")))
            (mu:open :file :input path))
          (prelude:raise path 'prelude:load "not a file designator"))))
