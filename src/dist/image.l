;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; image namespace
;;;
(mu:intern mu:*mu/null* "image" (mu:make-namespace "image"))

(mu:intern image "+version+" "0.0.2")
(mu:intern image "%modules%" (mu:make-namespace "%modules%"))
(mu:intern image "%eof%" (mu:make-symbol "eof"))

(mu:intern image "%mapc"
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

(mu:intern image "%assq"
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


(mu:intern image "%concat-strings"
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
;;; modules
;;;
(mu:intern image "%load-module-file"
   (:lambda (path lang)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop image:%eof%)
                image:%eof%
                ((:lambda (form)
                   (:if (mu:eq form image:%eof%)
                        image:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:eq () loop)))))
                 (mu:read stream () image:%eof%))))
         ()))
      (mu:open :file :input path :t))
     :t))

(mu:intern image "%load-module-def"
   (:lambda (path module)
     ((:lambda (stream)
        (mu:fix
         (:lambda (loop)
           (:if (mu:eq loop image:%eof%)
                image:%eof%
                ((:lambda (form)
                   (:if (mu:eq form image:%eof%)
                        image:%eof%
                        ((:lambda ()
                           (mu:eval (mu:compile form))
                           (mu:close stream)
                           (mu:eq () loop)))))
                 (mu:read stream () image:%eof%))))
         ()))
      (mu:open :file :input path :t))
     (mu:symbol-value (mu:find image:%modules% module))))

;;;
;;; provide/require
;;;
(mu:intern image "%provide"
   (:lambda (module property-list)
     (mu:intern image:%modules% module property-list)))

(mu:intern image "%require"
   (:lambda (module)
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
                         (image:%concat-strings `("/opt/mu/modules/" ,module "/" ,file-name))
                         (mu:cdr (image:%assq :lang module-def))))
                      files)
                     :t)
                   (mu:cdr (image:%assq :require module-def))
                   (mu:cdr (image:%assq :load module-def)))
                  (mu:raise module :open)))
           (image:%load-module-def
            (image:%concat-strings `("/opt/mu/modules/" ,module "/mod.def"))
            module)))))
