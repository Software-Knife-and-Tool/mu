;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; ARM compiler
;;;

;;;
;;; file compiler
;;;
(mu:intern :arm "compile-file"
  (:lambda (source dest verbose)
    ((:lambda (ifs ofs)
       (:if verbose
            (prelude:format :t ";;; compiling ~A~%" `(,source))
            ())
       (mu:fix
        (:lambda (loop)
          (:if (prelude:eof ifs)
               loop
               (prelude:with-exception
                (:lambda (ex)
                   (prelude:%exceptf mu:err-out "compile exception: ~A on ~A by ~S ~A~%" () ex)
                   loop)
                (:lambda ()
                   ((:lambda (form)
                       (:if (prelude:eof ifs)
                           loop
                           ((:lambda ()
                               (:if verbose
                                    (prelude:format :t "form: ~S~%" `(,form))
                                    ())
                               (prelude:format ofs " ~S~%" `(,(mu:compile form)))
                               (prelude:null loop)))))
                    (prelude:read ifs :t ()))))))
        ())
       (prelude:format ofs ")" ())
       (mu:close ifs)
       (mu:close ofs))
     (mu:open :file :input source)
     (mu:open :file :output dest))))
