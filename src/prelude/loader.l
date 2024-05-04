;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader utilities
;;;

;;;
;;; loader
;;;
(lib:intern prelude "load"
  (:lambda (path verbose)
    (:if (prelude:stringp path)
         ((:lambda (ifs)
            (:if (prelude:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (prelude:format :t ";;; loading ~A~%" `(,path))
                         ())
                    ((:lambda (status)
                       (:if (lib:eq prelude:%eof% status)
                            :t
                            ()))
                     (lib:fix
                        (:lambda (loop)
                           (:if (lib:eq prelude:%eof% loop)
                                prelude:%eof%
                                (prelude:with-exception
                                    (:lambda (except)
                                      (prelude:%exceptf lib:err-out "loader exception: ~A on ~A by ~S, ~A~%" () except))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (lib:eq prelude:%eof% form)
                                               prelude:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (prelude:format :t "~A~%" `(,value))
                                                       ())
                                                  (prelude:null loop))
                                                (lib:eval (prelude:compile form)))))
                                       (lib:read ifs () prelude:%eof%))))))
                       ()))))
                  (prelude:raise prelude:streamp ifs 'prelude:load "cannot open input file")))
            (lib:open :file :input path))
          (prelude:raise path 'prelude:load "not a file designator"))))
