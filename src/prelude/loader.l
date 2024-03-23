;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader utilities
;;;

;;;
;;; loader
;;;
(core:intern :prelude "load"
  (:lambda (path verbose)
    (:if (prelude:stringp path)
         ((:lambda (ifs)
            (:if (prelude:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (prelude:format :t ";;; loading ~A~%" `(,path))
                         ())
                    ((:lambda (status)
                       (:if (core:eq prelude:%eof% status)
                            :t
                            ()))
                     (core:fix
                        (:lambda (loop)
                           (:if (core:eq prelude:%eof% loop)
                                prelude:%eof%
                                (prelude:with-exception
                                    (:lambda (except)
                                      (prelude:%exceptf core:err-out "loader exception: ~A on ~A by ~S, ~A~%" () except))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (core:eq prelude:%eof% form)
                                               prelude:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (prelude:format :t "~A~%" `(,value))
                                                       ())
                                                  (prelude:null loop))
                                                (core:eval (prelude:compile form)))))
                                       (core:read ifs () prelude:%eof%))))))
                       ()))))
                  (prelude:raise prelude:streamp ifs 'prelude:load "cannot open input file")))
            (core:open :file :input path))
          (prelude:raise path 'prelude:load "not a file designator"))))
