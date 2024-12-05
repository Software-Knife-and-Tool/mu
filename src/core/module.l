;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; modules
;;;
(mu:intern core "modules"
   (:lambda ()
     (core:%mapcar mu:symbol-name (mu:namespace-symbols core:%modules%))))

(mu:intern core "modules-loaded"
   (:lambda ()
     (core:%mapcar mu:symbol-name (mu:namespace-symbols core:%modules-loaded%))))

;;;
;;; provide/require
;;;
(mu:intern core "provide"
   (:lambda (module property-list)
     (:if (core:stringp module)
          (:if (core:listp property-list)
              (mu:intern core:%modules% module property-list)
              (core:raise property-list 'core:provide "is not a property list"))
          (core:raise module 'core:provide "is not a module name"))))

(mu:intern core "require"
   (:lambda (module)
     (:if (core:stringp module)
          (:if (mu:find core:%modules% module)
               ()
               ((:lambda ()
                  (core:%load-lib-file (core:%format () "/opt/mu/modules/~A/mod.def" `(,module)))
                  ((:lambda (requires files)
                     (core:%mapc
                      (:lambda (module)
                        (core:require module))
                      requires)
                     (core:%mapc
                      (:lambda (file-name)
                        (core:%load-lib-file (core:%format () "/opt/mu/modules/~A/~A" `(,module ,file-name))))
                      files)
                     :t)
                   (mu:cdr (core:%assq :require (mu:symbol-value (mu:find core:%modules% module))))
                   (mu:cdr (core:%assq :load (mu:symbol-value (mu:find core:%modules% module))))))))
   (core:raise module 'core:require "is not a provided module"))))

;;;
;;; require-lib
;;;
(mu:intern core "%load-lib-file"
   (:lambda (path)
     (:if (core:stringp path)
          ((:lambda (stream)
             (mu:fix
              (:lambda (loop)
                (:if (mu:eq loop core:%eof%)
                     core:%eof%
                     ((:lambda (form)
                        (:if (mu:eq form core:%eof%)
                             core:%eof%
                             ((:lambda ()
                                (mu:eval (mu:compile form))
                                (core:null loop)))))
                      (mu:read stream () core:%eof%))))
              ()))
           (mu:open :file :input path))
          (core:raise path 'core:load-file "not a file path"))
     :t))
