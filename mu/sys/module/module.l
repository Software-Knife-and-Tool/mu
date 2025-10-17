;;; SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;; SPDX-License-Identifier: MIT

;;;
;;; modules
;;;
(mu:intern module "system/modules" (mu:make-namespace "system/modules"))

(mu:intern module "%string-concat"
   (:lambda (strings)
     ((:lambda (concat)
        (mu:fix
         (:lambda (list)
           (:if list
                ((:lambda (string)
                   (mu:fix
                    (:lambda (index)
                      (:if (mu:less-than index (mu:vector-length string))
                           ((:lambda ()
                              (mu:write-char (mu:svref string index) concat)
                              (mu:add index 1)))
                           index))
                    0)
                   (mu:cdr list))
                 (mu:car list))
                list))
         strings)
        (mu:get-string concat))
        (mu:open :string :output "" :t))))

(mu:intern module "module-version"
   (:lambda (module)
     (:if (core:stringp module)
          ((:lambda (module-def)
             (:if module-def
               (mu:cdr (core:%assq :version module-def))
               (mu:raise-from module 'module:module-version :range)))
           (mu:symbol-value (mu:find module:modules/ module)))
     (mu:raise-from module 'module:module-version :type))))

(mu:intern module "module-namespace"
   (:lambda (module)
     (:if (core:stringp module)
          ((:lambda (module-def)
             (:if module-def
                  (mu:cdr (core:%assq :ns module-def))
                  (mu:raise-from module 'module:module-namespace :range)))
           (mu:symbol-value (mu:find module:modules/ module)))
     (mu:raise-from module 'module:module-namespace :type))))

;;;
;;; utilities
;;;
(mu:intern module "%load-module-file"
   (:lambda (path lang)
     (:if (module:stringp path)
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
          (mu:raise-from path 'module:%load-module-file :type))
     :t))

(mu:intern module "%load-module-def"
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
          (mu:raise-from path 'module:%load-module-def :type))
     (mu:symbol-value (mu:find module:modules/ module))))

;;;
;;; provide/require
;;;
(mu:intern module "provide"
   (:lambda (module property-list)
     (:if (core:stringp module)
          (:if (core:listp property-list)
              (mu:intern core:modules/ module property-list)
              (mu:raise-from property-list 'core:provide :type))
          (mu:raise-from module 'module:provide :type))))

(mu:intern module "require"
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
                             (module:%load-module-file
                              (core:%string-concat `("/opt/mu/modules/" ,module "/" ,file-name))
                              (mu:cdr (core:%assq :lang module-def))))
                           files)
                          :t)
                        (mu:cdr (core:%assq :require module-def))
                        (mu:cdr (core:%assq :load module-def))
                        (mu:cdr (core:%assq :ns module-def)))
                       (mu:raise-from module 'core:require :open)))
                  (core:%load-module-def
                   (core:%string-concat `("/opt/mu/modules/" ,module "/mod.def"))
                   module)))
          (mu:raise-from module 'module:require :type))))

(mu:intern module "%require"
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
                       (mu:raise-from module 'core:require :open)))
                  (core:%load-module-def
                   (core:%string-concat `(,base "/" ,module "/mod.def"))
                   module)))
          (mu:raise-from module 'core:%require :type))))
