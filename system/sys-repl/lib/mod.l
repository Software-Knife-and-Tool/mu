;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core-sys namespace
;;;
(mu:intern mu:*null/* "core-sys" (mu:make-namespace "core-sys"))

(mu:intern core-sys "+version+" "0.0.3")
(mu:intern core-sys "*core-sys/modules*" (mu:make-namespace "core-sys/modules"))


;;;
;;; parser
;;;
(mu:intern core-sys "%core-sys-parse"
   (:lambda (stream lang)
     ((:lambda (form)
        form)
      (:if (mu:eq :core lang)
           ((:lambda (reader)
              (mu:apply (mu:symbol-value reader) `(,stream () ,core:%eof%)))
            (mu:find (mu:find-namespace "core") "read"))
           (mu:read stream () core:%eof%)))))

;;;
;;; modules
;;;
(mu:intern core-sys "%load-module-file"
   (:lambda (path output-stream lang)
     ((:lambda (input-stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop core-sys:%eof%)
                core-sys:%eof%
                ((:lambda (form)
                   (:if (mu:eq form core-sys:%eof%)
                        core-sys:%eof%
                        ((:lambda ()
                           (mu:write form :t output-stream)
                           (mu:eq () loop)))))
                 (core-sys:%core-sys-parse input-stream lang))))
         ()))
      (mu:open :file :input path :t))
     :t))

(mu:intern core-sys "%load-module-def"
   (:lambda (path module)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop core-sys:%eof%)
                core-sys:%eof%
                ((:lambda (form)
                   (:if (mu:eq form core-sys:%eof%)
                        core-sys:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:close stream)
                           (mu:eq () loop)))))
                 (mu:read stream () core-sys:%eof%))))
         ()))
      (mu:open :file :input path :t))
     (mu:symbol-value (mu:find core-sys:*core-sys/modules* module))))

;;;
;;; provide/require
;;;
(mu:intern core-sys "provide"
   (:lambda (module property-list)
     (mu:intern core-sys:*core-sys/modules* module property-list)))

(mu:intern core-sys "require"
   (:lambda (base module)
     (:if (mu:find core-sys:*core-sys/modules* module)
          ()
          ((:lambda (module-def output-stream)
             (:if module-def
                  ((:lambda (requires files ns lang)
                     (core:%mapc
                      (:lambda (module)
                        (core-sys:require module))
                      requires)

                     (mu:write
                      (core:%string-concat
                       `(
                         "(mu:intern mu:*null/* \""
                         ,module
                         "\" (mu:make-namespace \""
                         ,module
                         "\"))"                         ))
                      ()
                      output-stream)
                     
                     (mu:make-namespace ns)
                     (core:%mapc
                      (:lambda (file-name)
                        (core-sys:%load-module-file
                         (core-sys:%concat-strings `(,base ,module "/" ,file-name))
                         output-stream
                         lang))
                      files)
                     (mu:close output-stream)
                     :t)
                   (mu:cdr (core:%assq :require module-def))
                   (mu:cdr (core:%assq :load module-def))
                   (mu:cdr (core:%assq :ns module-def))
                   (mu:cdr (core:%assq :lang module-def)))
                  (mu:raise module :open)))
           (core-sys:%load-module-def
            (core-sys:%concat-strings `(,base ,module "/mod.def"))
            module)
           (mu:open :file :output (core-sys:%concat-strings `(,module ".core-sys")) :t)))
     :t))
