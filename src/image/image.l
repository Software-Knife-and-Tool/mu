;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; image namespace
;;;
(mu:intern mu:%null-ns% "image" (mu:make-namespace "image"))

(mu:intern image "+version+" "0.0.1")
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

;;;
;;; replace this with a path builder
;;;
(mu:intern image "%fmt-op"
   (:lambda (dir dest arg-list)
     ((:lambda (fn)
        (mu:apply fn `(,(mu:car arg-list))))
     (mu:cdr
      (image:%assq
       dir
       `(,(mu:cons #\A (:lambda (arg) (mu:write arg () dest) (mu:cdr arg-list)))
         ,(mu:cons #\S (:lambda (arg) (mu:write arg :t dest) (mu:cdr arg-list)))
         ,(mu:cons #\T (:lambda (arg) (mu:write-char #\tab dest) arg-list))
         ,(mu:cons #\~ (:lambda (arg) (mu:write-char #\~ dest) arg-list))
         ,(mu:cons #\% (:lambda (arg) (mu:write-char #\linefeed arg dest) arg-list))))))))

(mu:intern image "%fmt-loop"
   (:lambda (dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:vector-length fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       (mu:cons (mu:add 2 nth) (image:%fmt-op (mu:svref fmt (mu:add 1 nth)) dest argl))
                       ((:lambda ()
                           (mu:write-char ch dest)
                           (mu:cons (mu:add 1 nth) argl)))))
                (mu:svref fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     (mu:cons 0 arg-list))
    (mu:get-string dest)))

(mu:intern image "%fmt"
   (:lambda (format-string arg-list)
     (image:%fmt-loop
      (mu:open :string :output "" :t)
      format-string
      arg-list)))

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
                         (image:%fmt "/opt/mu/modules/~A/~A" `(,module ,file-name))
                         (mu:cdr (image:%assq :lang module-def))))
                      files)
                     :t)
                   (mu:cdr (image:%assq :require module-def))
                   (mu:cdr (image:%assq :load module-def)))
                  (mu:raise module :open)))
           (image:%load-module-def
            (image:%fmt "/opt/mu/modules/~A/mod.def" `(,module))
            module)))))
