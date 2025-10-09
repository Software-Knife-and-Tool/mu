;;; SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;; SPDX-License-Identifier: MIT

;;;
;;; modules
;;;
(mu:intern core "module-version"
   (:lambda (module)
     (:if (core:stringp module)
          ((:lambda (module-def)
             (:if module-def
               (mu:cdr (core:%assq :version module-def))
               (core:%raise module 'core:module-version "no such module")))
           (mu:symbol-value (mu:find core:modules/ module)))
     (core:%raise module 'core:module-version "not a module name"))))

(mu:intern core "module-namespace"
   (:lambda (module)
     (:if (core:stringp module)
          ((:lambda (module-def)
             (:if module-def
                  (mu:cdr (core:%assq :ns module-def))
                  (core:%raise module 'core:module-namespace "no such module")))
           (mu:symbol-value (mu:find core:modules/ module)))
     (core:%raise module 'core:module-namespace "not a module name"))))

;;;
;;; utilities
;;;
(mu:intern core "%load-module-file"
   (:lambda (path lang)
     (:if (core:stringp path)
          (:if (mu:eq :core lang)
               (core:load path)
               ((:lambda (stream)
                  (mu:fix
                   (:lambda (loop)
                     (:if (mu:eq core:%eof% loop)
                          core:%eof%
                          ((:lambda (form)
                             (:if (mu:eq core:%eof% form)
                                  core:%eof%
                                  (core:%prog2
                                    (mu:eval (mu:compile form))
                                    (core:null loop))))
                           (mu:read stream () core:%eof%))))
                   ()))
                (mu:open :file :input path :t)))
          (core:%raise path 'core:%load-module-file "not a file path"))
     :t))

(mu:intern core "%load-module-def"
   (:lambda (path module)
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
                                (mu:close stream)
                                (core:null loop)))))
                      (mu:read stream () core:%eof%))))
              ()))
           (mu:open :file :input path :t))
          (core:%raise path 'core:%load-module-def "not a file path"))
     (mu:symbol-value (mu:find core:modules/ module))))

;;;
;;; provide/require
;;;
(mu:intern core "provide"
   (:lambda (module property-list)
     (:if (core:stringp module)
          (:if (core:listp property-list)
              (mu:intern core:modules/ module property-list)
              (core:%raise property-list 'core:provide "is not a property list"))
          (core:%raise module 'core:provide "is not a module name"))))

(mu:intern core "require"
   (:lambda (module)
     (:if (core:stringp module)
          (:if (mu:find core:modules/ module)
               ()
               ((:lambda (module-def)
                  (:if module-def
                       ((:lambda (requires files ns)
                          (core:%mapc
                           (:lambda (module)
                             (core:require module))
                           requires)
                          (:if (mu:find-namespace ns)
                               ()
                               (mu:make-namespace ns))
                          (core:%mapc
                           (:lambda (file-name)
                             (core:%load-module-file
                              (core:%string-concat `("/opt/mu/modules/" ,module "/" ,file-name))
                              (mu:cdr (core:%assq :lang module-def))))
                           files)
                          :t)
                        (mu:cdr (core:%assq :require module-def))
                        (mu:cdr (core:%assq :load module-def))
                        (mu:cdr (core:%assq :ns module-def)))
                       (core:%raise module 'core:require "cannot open module-def")))
                  (core:%load-module-def
                   (core:%string-concat `("/opt/mu/modules/" ,module "/mod.def"))
                   module)))
          (core:%raise module 'core:require "is not a module name"))))

(mu:intern core "%require"
   (:lambda (module base)
     (:if (core:stringp module)
          (:if (mu:find core:modules/ module)
               ()
               ((:lambda (module-def)
                  (:if module-def
                       ((:lambda (requires files ns)
                          (core:%mapc
                           (:lambda (module)
                             (core:require module))
                           requires)
                          (:if (mu:find-namespace ns)
                               ()
                               (mu:make-namespace ns))
                          (core:%mapc
                           (:lambda (file-name)
                             (core:%load-module-file
                              (core:%string-concat `(,base  "/" ,module "/" ,file-name))
                              (mu:cdr (core:%assq :lang module-def))))
                           files)
                          :t)
                        (mu:cdr (core:%assq :require module-def))
                        (mu:cdr (core:%assq :load module-def))
                        (mu:cdr (core:%assq :ns module-def)))
                       (core:%raise module 'core:require "cannot open module-def")))
                  (core:%load-module-def
                   (core:%string-concat `(,base "/" ,module "/mod.def"))
                   module)))
          (core:%raise module 'core:%require "is not a module name"))))
