;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; ARM compiler
;;;

;;;
;;; file compiler
;;;
(lib:intern :arm "compile-file"
  (:lambda (source dest verbose)
    ((:lambda (ifs ofs)
       (:if verbose
            (prelude:format :t ";;; compiling ~A~%" `(,source))
            ())
       (lib:fix
        (:lambda (loop)
          (:if (prelude:eof ifs)
               loop
               (prelude:with-exception
                (:lambda (ex)
                   (prelude:%exceptf lib:error-output "compile exception: ~A on ~A by ~S ~A~%" () ex)
                   loop)
                (:lambda ()
                   ((:lambda (form)
                       (:if (prelude:eof ifs)
                           loop
                           ((:lambda ()
                               (:if verbose
                                    (prelude:format :t "form: ~S~%" `(,form))
                                    ())
                               (prelude:format ofs " ~S~%" `(,(lib:compile form)))
                               (prelude:null loop)))))
                    (prelude:read ifs :t ()))))))
        ())
       (prelude:format ofs ")" ())
       (lib:close ifs)
       (lib:close ofs))
     (lib:open :file :input source)
     (lib:open :file :output dest))))
