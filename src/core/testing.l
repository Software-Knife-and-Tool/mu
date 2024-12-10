;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; module testing
;;;
(mu:intern mu:%null-ns% "core" (mu:make-namespace "core"))

(mu:intern core "+version+" "0.0.1")

(mu:intern core "test-require"
   (:lambda (module src)
     (:if (core:stringp module)
          (:if (mu:find core:%modules% module)
               ()
               ((:lambda (module-def)
                  (:if module-def
                       ((:lambda (requires files)
                          (core:%mapc
                           (:lambda (module)
                             (core:require module))
                           requires)
                          (core:%mapc
                           (:lambda (file-name)
                             (core:%load-module-file
                              (core:%format () "~A/~A/~A" `(,src ,module ,file-name))
                              (mu:cdr (core:%assq :lang module-def))))
                           files)
                          :t)
                        (mu:cdr (core:%assq :require module-def))
                        (mu:cdr (core:%assq :load module-def)))
                       (core:raise module 'core:require "cannot open module-def")))
                  (core:%load-module-def
                   (core:%format () "/opt/mu/modules/~A/mod.def" `(,module))
                   module)))
          (core:raise module 'core:require "is not a module name"))))
