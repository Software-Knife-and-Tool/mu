;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fasl namespace
;;;
(mu:intern mu:*mu/null* "fasl" (mu:make-namespace "fasl"))

(mu:intern fasl "+version+" "0.0.3")
(mu:intern fasl "*fasl/modules*" (mu:make-namespace "fasl/modules"))
(mu:intern fasl "%eof%" (mu:make-symbol "eof"))

(mu:intern fasl "debug"
   (:lambda (obj)
     (mu:write obj :t mu:*standard-output*)
     (mu:write-char #\linefeed mu:*standard-output*)))

(mu:intern fasl "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (mu:apply function (mu:cons (mu:car list-cdr) ()))
                 (mu:cdr list-cdr)))
             ()))
      list)
     list))

(mu:intern fasl "%assq"
   (:lambda (item alist)
      (mu:car
       (mu:fix
        (:lambda (list)
           (:if list
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:car entry))
                              list
                              cdr)
                         cdr))
                 (mu:car list)
                 (mu:cdr list))
                ()))
        alist))))

(mu:intern fasl "%concat-strings"
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

;;;
;;; parser
;;;
(mu:intern fasl "%fasl-parse"
   (:lambda (stream lang)
     ((:lambda (form)
        form)
      (:if (mu:eq :core lang)
           ((:lambda (reader)
              (mu:apply (mu:symbol-value reader) `(,stream () ,fasl:%eof%)))
            (mu:find (mu:find-namespace "core") "read"))
           (mu:read stream () fasl:%eof%)))))

;;;
;;; modules
;;;
(mu:intern fasl "%load-module-file"
   (:lambda (path output-stream lang)
     ((:lambda (input-stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop fasl:%eof%)
                fasl:%eof%
                ((:lambda (form)
                   (:if (mu:eq form fasl:%eof%)
                        fasl:%eof%
                        ((:lambda ()
                           (mu:write form :t output-stream)
                           (mu:eq () loop)))))
                 (fasl:%fasl-parse input-stream lang))))
         ()))
      (mu:open :file :input path :t))
     :t))

(mu:intern fasl "%load-module-def"
   (:lambda (path module)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop fasl:%eof%)
                fasl:%eof%
                ((:lambda (form)
                   (:if (mu:eq form fasl:%eof%)
                        fasl:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:close stream)
                           (mu:eq () loop)))))
                 (mu:read stream () fasl:%eof%))))
         ()))
      (mu:open :file :input path :t))
     (mu:symbol-value (mu:find fasl:*fasl/modules* module))))

;;;
;;; provide/require
;;;
(mu:intern fasl "provide"
   (:lambda (module property-list)
     (mu:intern fasl:*fasl/modules* module property-list)))

(mu:intern fasl "require"
   (:lambda (base module)
     (:if (mu:find fasl:*fasl/modules* module)
          ()
          ((:lambda (module-def output-stream)
             (:if module-def
                  ((:lambda (requires files ns lang)
                     (fasl:%mapc
                      (:lambda (module)
                        (fasl:require module))
                      requires)
                     (mu:write
                      (fasl:%concat-strings
                       `("(mu:intern mu:*mu/null* "
                         "\""
                         ,module
                         "\" "
                         "(mu:make-namespace "
                         "\""
                         ,module
                         "\"))"))
                      ()
                     output-stream)
                     (mu:make-namespace ns)
                     (fasl:%mapc
                      (:lambda (file-name)
                        (fasl:%load-module-file
                         (fasl:%concat-strings `(,base ,module "/" ,file-name))
                         output-stream
                         lang))
                      files)
                     (mu:close output-stream)
                     :t)
                   (mu:cdr (fasl:%assq :require module-def))
                   (mu:cdr (fasl:%assq :load module-def))
                   (mu:cdr (fasl:%assq :ns module-def))
                   (mu:cdr (fasl:%assq :lang module-def)))
                  (mu:raise module :open)))
           (fasl:%load-module-def
            (fasl:%concat-strings `(,base ,module "/fasl.def"))
            module)
           (mu:open :file :output (fasl:%concat-strings `(,module ".fasl")) :t)))
     :t))
