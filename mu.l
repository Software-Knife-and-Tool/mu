(lib:intern (lib:find-ns "") "list" (lambda (&rest list) list))

(lib:intern (lib:find-ns "") "list*"
   (lambda (&rest args)
     (lib:car
      (lib:fix
       (:lambda (loop)
         ((:lambda (list args)
            (:if args
                 (:if (lib:cdr args)
                      (lib:cons (lib:append list (lib:cons (lib:car args) ())) (lib:cdr args))
                      (lib:cons (lib:append list (lib:car args)) ()))
                 loop))
          (lib:car loop)
          (lib:cdr loop)))
       (lib:cons () args)))))

(lib:intern (lib:find-ns "") "list+"
   (lambda (&rest args)
     (if args
         (if (lib:cdr args)
             (lib:cons (lib:car args) (list+ (lib:cdr args)))
             (lib:car args)))))

(define-macro progn (&rest body)
  (list (list* 'lambda () body)))

#|
(define-macro when (test &rest body)
  (list 'if test (list* 'progn body)))

(define-macro unless (test &rest body)
  (list 'if test () (list* 'progn body)))
|#
