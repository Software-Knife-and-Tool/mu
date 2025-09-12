;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnum functions
;;;

;;;
;;; generics
;;;
(%defgeneric 1+ (:fixnum . (:lambda (n) (mu:add 1 n))) (:float . (:lambda (f) (mu:fadd 1.0 f))))
(%defgeneric 1- (:fixnum . (:lambda (n) (mu:sub 1 n))) (:float . (:lambda (f) (mu:fsub 1.0 f))))

;;;
;;; min &rest reals+ => min-real
;;;
(mu:intern common "min"
   (core:compile
    '(%lambda (real &rest reals)
      (:if reals
       (core:%foldl
        (:lambda (el acc)
          (:if (core:numberp el)
               (:if (mu:less-than el acc)
                    el
                    acc)
               (core:error "min: invalid argument, ~A~%" `(el))))
        (mu:car reals)
        (mu:cdr reals))
       real))))

;;;
;;;  max &rest reals+ => max-real
;;;
(mu:intern common "max"
   (core:compile
    '(%lambda (real &rest reals)
      (:if reals       
       (core:%foldl
        (:lambda (el acc)
          (:if (core:numberp el)
               (:if (mu:less-than acc el)
                    el
                    acc)
               (core:error "max: invalid argument, ~A~%" `(,el))))
        (mu:car reals)
        (mu:cdr reals))
       real))))



;;; round to negative infinity
(mu:intern common "truncate"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:error "truncate: not a fixnum, ~A~%" `(,m)))
          (core:error "truncate: not a fixnum, ~A~%" `(,n)))))


;;; round to negative infinity
(mu:intern common "floor"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:error "floor: not a fixnum, ~A~%" `(,m)))
          (core:error "floor: not a fixnum, ~A~%" `(n)))))


;;; round to positive infinity
(mu:intern common "ceiling"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:error "ceiling: not a fixnum, ~A~%" `(,m)))
          (core:error "ceiling: not a fixnum" `(,n)))))



;;; round to nearest
(mu:intern common "round"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:sub (mu:div n m) (mu:sub n (mu:mul m (mu:div n m))))
               (core:error "round: not a fixnum, ~A~%" `(,m)))
          (core:error "round: not a fixnum, ~A~%" `(,n)))))

#|
(mu:intern common "mod"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:floor m n))
               (core:error "mod: not a fixnum, ~A~%" `(,m)))
          (core:error "mod: not a fixnum, ~A~%", `(,n)))))
|#

(mu:intern common "rem"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:truncate m n))
               (core:error "rem: not a fixnum, ~A~%" `(,m)))
          (core:error "rem: not a fixnum, ~A~%" `(,n)))))
