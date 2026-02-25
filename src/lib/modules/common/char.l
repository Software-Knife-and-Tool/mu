;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; chars
;;;
(mu:intern mu:*null/* "char-int"
   (:lambda (char)
     (:if (core:charp char)
          (mu:svref (mu:repr char) 1)
          (core:error "char-int: is not a char, ~A~%" `(,char)))))

(mu:intern mu:*null/* "int-char"
   (:lambda (int)
     (:if (core:fixnump int)
          (mu:unrepr (mu:make-vector :byte `(32 ,int 0 0 0 0 0 0)))
          (core:error "int-char: is not a fixnum, ~A~%" `(,int)))))
