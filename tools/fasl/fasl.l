;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fasl format:
;;;
;;;     vector of tag fixnums
;;;
(mu:intern (mu:find-namespace "") "fasl" (mu:make-namespace "fasl"))

(mu:intern fasl "fasl"
  (:lambda (path verbose)
    (:if (prelude:stringp path)
         ((:lambda (ifs)
            (:if (prelude:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (prelude:format :t ";;; fasl loading ~A~%" `(,path))
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
                                       "fasl loader exception: ~A on ~A by ~S, ~A~%"
                                       ()
                                       exception))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (mu:eq prelude:%eof% form)
                                               prelude:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (prelude:format :t "~A~%" `(,(mu:repr :vector value)))
                                                       ())
                                                  (prelude:null loop))
                                                (mu:eval (prelude:compile form)))))
                                       (mu:read ifs () prelude:%eof%))))))
                       ()))))
                  (prelude:raise prelude:streamp ifs 'prelude:load "fasl: cannot open input file")))
            (mu:open :file :input path))
          (prelude:raise path 'prelude:load "fasl: not a file designator"))))
