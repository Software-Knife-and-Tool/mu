;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; ARM image loader
;;;

;;;
;;; loader
;;;
#|
(mu:intern :arm "load"
  (:lambda (source verbose)
    ;;; (prelude:raise prelude:stringp source 'prelude:load "not a file designator")
    ((:lambda (ifs)
       ;;; (prelude:raise prelude:streamp ifs 'prelude:load "cannot open input file")
       (:if verbose
            (prelude:format :t ";;; loading ~A~%" `(,source))
            ())
       (mu:fix
        (:lambda (loop)
          (:if (prelude:eof ifs)
               loop
               (prelude:with-exception
                (:lambda (exception)
                     (prelude:%exceptf mu:err-out "load exception: ~A~%" () exception))
                (:lambda ()
                   ((:lambda (form)
                      (:if (prelude:eof ifs)
                           loop
                           (:if verbose
                                (prelude:format :t "~A~%" `(,form))
                                ()))
                      (prelude:null loop))
                    (mu:eval (prelude:compile (prelude:read ifs :t ()))))))))
        ()))
(mu:open :file :input source))))
|#
