;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mcore namespace
;;;
(mu:intern mu:*null/* "mcore" (mu:make-namespace "mcore"))

(mu:intern mcore "+version+" "0.0.3")
(mu:intern mcore "*mcore/modules*" (mu:make-namespace "mcore/modules"))


;;;
;;; parser
;;;
(mu:intern mcore "%mcore-parse"
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
(mu:intern mcore "%load-module-file"
   (:lambda (path output-stream lang)
     ((:lambda (input-stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop mcore:%eof%)
                mcore:%eof%
                ((:lambda (form)
                   (:if (mu:eq form mcore:%eof%)
                        mcore:%eof%
                        ((:lambda ()
                           (mu:write form :t output-stream)
                           (mu:eq () loop)))))
                 (mcore:%mcore-parse input-stream lang))))
         ()))
      (mu:open :file :input path :t))
     :t))

(mu:intern mcore "%load-module-def"
   (:lambda (path module)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop mcore:%eof%)
                mcore:%eof%
                ((:lambda (form)
                   (:if (mu:eq form mcore:%eof%)
                        mcore:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:close stream)
                           (mu:eq () loop)))))
                 (mu:read stream () mcore:%eof%))))
         ()))
      (mu:open :file :input path :t))
     (mu:symbol-value (mu:find mcore:*mcore/modules* module))))

;;;
;;; provide/require
;;;
(mu:intern mcore "provide"
   (:lambda (module property-list)
     (mu:intern mcore:*mcore/modules* module property-list)))

(mu:intern mcore "require"
   (:lambda (base module)
     (:if (mu:find mcore:*mcore/modules* module)
          ()
          ((:lambda (module-def output-stream)
             (:if module-def
                  ((:lambda (requires files ns lang)
                     (core:%mapc
                      (:lambda (module)
                        (mcore:require module))
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
                        (mcore:%load-module-file
                         (mcore:%concat-strings `(,base ,module "/" ,file-name))
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
           (mcore:%load-module-def
            (mcore:%concat-strings `(,base ,module "/mod.def"))
            module)
           (mu:open :file :output (mcore:%concat-strings `(,module ".mcore")) :t)))
     :t))
