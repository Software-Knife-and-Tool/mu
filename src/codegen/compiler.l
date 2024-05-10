;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; ARM compiler
;;;

;;;
;;; file compiler
;;;
(core:intern :arm "compile-file"
  (:lambda (source dest verbose)
    ((:lambda (ifs ofs)
       (:if verbose
            (prelude:format :t ";;; compiling ~A~%" `(,source))
            ())
       (core:fix
        (:lambda (loop)
          (:if (prelude:eof ifs)
               loop
               (prelude:with-exception
                (:lambda (ex)
                   (prelude:%exceptf core:error-output "compile exception: ~A on ~A by ~S ~A~%" () ex)
                   loop)
                (:lambda ()
                   ((:lambda (form)
                       (:if (prelude:eof ifs)
                           loop
                           ((:lambda ()
                               (:if verbose
                                    (prelude:format :t "form: ~S~%" `(,form))
                                    ())
                               (prelude:format ofs " ~S~%" `(,(core:compile form)))
                               (prelude:null loop)))))
                    (prelude:read ifs :t ()))))))
        ())
       (prelude:format ofs ")" ())
       (core:close ifs)
       (core:close ofs))
     (core:open :file :input source)
     (core:open :file :output dest))))
