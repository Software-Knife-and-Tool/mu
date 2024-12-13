;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; test require
;;;
(mu:intern image "%test-require"
   (:lambda (module src)
     (:if (mu:find image:%modules% module)
          ()
          ((:lambda (module-def)
             (:if module-def
                  ((:lambda (requires files)
                     (image:%mapc
                      (:lambda (module)
                        (image:%require module))
                      requires)
                     (image:%mapc
                      (:lambda (file-name)
                        (image:%load-module-file
                         (image:%fmt "~A/~A/~A" `(,src ,module ,file-name))
                         (mu:cdr (image:%assq :lang module-def))))
                      files)
                     :t)
                   (mu:cdr (image:%assq :require module-def))
                   (mu:cdr (image:%assq :load module-def)))
                  (mu:raise module :open)))
           (image:%load-module-def
            (image:%fmt "~A/~A/mod.def" `(,src ,module))
            module)))))
