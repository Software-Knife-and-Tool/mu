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
    '(%lambda (&rest reals)
      (:if reals       
       (core:%foldl
        (:lambda (el acc)
          (:if (core:numberp el)
               (:if (mu:less-than el acc)
                    el
                    acc)
               (core:raise el 'common:min "invalid argument")))
        (mu:car reals)
        (mu:cdr reals))
       (core:raise () 'common:min "invalid argument")))))

;;;
;;;  max &rest reals+ => max-real
;;;
(mu:intern common "max"
   (core:compile
    '(%lambda (&rest reals)
      (:if reals       
       (core:%foldl
        (:lambda (el acc)
          (:if (core:numberp el)
               (:if (mu:less-than acc el)
                    el
                    acc)
               (core:raise el 'common:max "invalid argument")))
        (mu:car reals)
        (mu:cdr reals))
       (core:raise () 'common:max "invalid argument")))))

;;; round to negative infinity
(mu:intern common "truncate"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:truncate "not a fixnum"))
          (core:raise n 'common:truncate "not a fixnum"))))

;;; round to negative infinity
(mu:intern common "floor"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:rem "not a fixnum"))
          (core:raise n 'common:floor "not a fixnum"))))

;;; round to positive infinity
(mu:intern common "ceiling"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:ceiling "not a fixnum"))
          (core:raise n 'common:ceiling "not a fixnum"))))

;;; round to nearest
(mu:intern common "round"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:sub (mu:div n m) (mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:round "not a fixnum"))
          (core:raise n 'common:round "not a fixnum"))))

(mu:intern common "mod"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:floor m n))
               (core:raise m 'common:mod "not a fixnum"))
          (core:raise n 'common:mod "not a fixnum"))))

(mu:intern common "rem"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:truncate m n))
               (core:raise m 'common:rem "not a fixnum"))
          (core:raise n 'common:rem "not a fixnum"))))
