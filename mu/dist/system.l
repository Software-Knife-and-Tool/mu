;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; system namespace
;;;
(mu:intern mu:*null/* "system" (mu:make-namespace "system"))

(mu:intern system "+version+" "0.0.3")
(mu:intern system "system/modules" (mu:make-namespace "system/modules"))
(mu:intern system "%eof%" (mu:make-symbol "eof"))

(mu:intern system "debug"
   (:lambda (obj)
     (mu:write obj :t mu:*standard-output*)
     (mu:write-char #\linefeed mu:*standard-output*)))

(mu:intern system "%mapc"
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

(mu:intern system "%assq"
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

(mu:intern system "%concat-strings"
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
(mu:intern system "%system-parse"
   (:lambda (stream lang)
     ((:lambda (form)
        form)
      (:if (mu:eq :core lang)
           ((:lambda (reader)
              (mu:apply (mu:symbol-value reader) `(,stream () ,system:%eof%)))
            (mu:find (mu:find-namespace "core") "read"))
           (mu:read stream () system:%eof%)))))

;;;
;;; modules
;;;
(mu:intern system "%load-system-file"
   (:lambda (path output-stream lang)
     ((:lambda (input-stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop system:%eof%)
                system:%eof%
                ((:lambda (form)
                   (:if (mu:eq form system:%eof%)
                        system:%eof%
                        ((:lambda ()
                           (mu:write form :t output-stream)
                           (mu:eq () loop)))))
                 (system:%system-parse input-stream lang))))
         ()))
      (mu:open :file :input path :t))
     :t))

(mu:intern system "%load-system-def"
   (:lambda (path module)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop system:%eof%)
                system:%eof%
                ((:lambda (form)
                   (:if (mu:eq form system:%eof%)
                        system:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:close stream)
                           (mu:eq () loop)))))
                 (mu:read stream () system:%eof%))))
         ()))
      (mu:open :file :input path :t))
     (mu:symbol-value (mu:find system:system/modules module))))

;;;
;;; provide/require
;;;
(mu:intern system "provide"
   (:lambda (module property-list)
     (mu:intern system:system/modules module property-list)))

(mu:intern system "require"
   (:lambda (base module)
     (:if (mu:find system:system/modules module)
          ()
          ((:lambda (module-def output-stream)
             (:if module-def
                  ((:lambda (requires files ns lang)
                     (system:%mapc
                      (:lambda (module)
                        (system:require module))
                      requires)
                     (mu:write
                      (system:%concat-strings
                       `("(mu:intern mu:*null/* "
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
                     (system:%mapc
                      (:lambda (file-name)
                        (system:%load-system-file
                         (system:%concat-strings `(,base ,module "/" ,file-name))
                         output-stream
                         lang))
                      files)
                     (mu:close output-stream)
                     :t)
                   (mu:cdr (system:%assq :require module-def))
                   (mu:cdr (system:%assq :load module-def))
                   (mu:cdr (system:%assq :ns module-def))
                   (mu:cdr (system:%assq :lang module-def)))
                  (mu:raise module :open)))
           (system:%load-system-def
            (system:%concat-strings `(,base ,module "/sys.def"))
            module)
           (mu:open :file :output (system:%concat-strings `(,module ".sys")) :t)))
     :t))
